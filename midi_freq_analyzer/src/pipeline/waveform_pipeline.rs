// src/pipeline/waveform_pipeline.rs

use crate::buffer::AudioBuffer;
use crate::analytics::waveform_analytics::WaveformAnalytics;
use crate::gui::waveform_gui::WaveformGui;
use crate::analytics::waveform_analytics::Waveform;
use std::time::{Instant, Duration};
use rustfft::{FftPlanner, num_complex::Complex};
use std::io::{stdout, Write};
use crate::analytics::note_label::frequency_to_note; // frequency_to_note
use crate::cli_log::log_status; // log_status 


pub struct WaveformPipeline {
    analytics: WaveformAnalytics,
    gui: WaveformGui,
    recent_peaks: Vec<f32>,
    last_y_update: Instant,
    smoothed_y: f32,
    avg_freq: f32,
    avg_bin: f32,
    last_cli_update: Instant,
    noise_floor_level: u8, // 0 = low, 1 = med, 2 = high
}

impl WaveformPipeline {

    pub fn new() -> Self {
        Self {
            analytics: WaveformAnalytics::new(),
            gui: WaveformGui::new(),
            recent_peaks: Vec::with_capacity(100),
            last_y_update: Instant::now(),
            smoothed_y: 0.01,
            avg_freq: 0.0,
            avg_bin: 0.0,
            last_cli_update: Instant::now(),
            noise_floor_level: 1,
        }
    }

    pub fn update(&mut self, buffer: &AudioBuffer) {
        let waveform = self.analytics.process(buffer);
        self.gui.display(&waveform);
    }

    pub fn gui(&self) -> &WaveformGui {
        &self.gui
    }

    pub fn analytics(&self) -> &WaveformAnalytics {
        &self.analytics
    }

    pub fn update_return(&self, buffer: &AudioBuffer) -> Waveform {
        self.analytics.process(buffer)
    }

    pub fn latest_peak(&mut self, buffer: &AudioBuffer) -> f32 {
        let waveform = self.analytics.process(buffer);
        let peak = waveform
            .samples
            .iter()
            .copied()
            .map(f32::abs)
            .fold(0.0, f32::max);
        if self.recent_peaks.len() >= 100 {
            self.recent_peaks.remove(0);
        }
        self.recent_peaks.push(peak);
        peak
    }

    pub fn y_range(&mut self) -> f32 {
        let now = Instant::now();
        let max = self.recent_peaks.iter().copied().fold(0.0, f32::max);
        let target = (max * 1.2).clamp(0.001, 1.0);
    
        let elapsed = now.duration_since(self.last_y_update);
        let rise_fast = elapsed >= Duration::from_millis(100);
        let fall_slow = elapsed >= Duration::from_secs(10);
    
        if rise_fast && target > self.smoothed_y {
            self.smoothed_y = target;
            self.last_y_update = now;
        } else if fall_slow && target < self.smoothed_y {
            self.smoothed_y *= 0.95;
            self.last_y_update = now;
        }
    
        self.smoothed_y
    }
            
    
    pub fn dominant_frequency(&mut self, buffer: &AudioBuffer) -> (f32, usize) {
        let samples = &buffer.samples;
        if samples.iter().all(|&x| x.abs() < 1e-6) {
            return (0.0, 0);
        }
    
        // let len = samples.len().next_power_of_two();
        // let len = 16_384;
        let len = samples.len().next_power_of_two() * 64 *2;

        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        fft.process(&mut input);
    
        // detect second frequency by isolatin window around first:
        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
        
        let mut mags = magnitudes.clone();
        let primary = mags
            .iter()
            .take(len / 2)
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Zero out ±2 bins around primary
        for i in primary.saturating_sub(10)..=(primary + 100).min(len / 2 - 1) {
            mags[i] = 0.0;
        }        

        let secondary = mags
            .iter()
            .take(len / 2)
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        // detect second frequency by isolatin window around first - End.

        // convert secondary peak bin to Hz and note name:
        let secondary_shift = 0.0; // You can later apply interpolation like primary
        let secondary_bin = secondary as f32 + secondary_shift;

        let sample_rate = 48000.0;
        let bin_w = sample_rate / len as f32;

        let secondary_freq = (secondary_bin * sample_rate) / len as f32;
        let secondary_note = frequency_to_note(secondary_freq);

        // convert secondary peak bin to Hz and note name - End.


        let (peak_bin, _) = input
            .iter()
            .take(len / 2)
            .enumerate()
            .max_by(|a, b| a.1.norm().partial_cmp(&b.1.norm()).unwrap())
            .unwrap_or((0, &Complex { re: 0.0, im: 0.0 }));
    
        // Sub-bin interpolation
        let mag = |i: usize| input.get(i).map(|c| c.norm()).unwrap_or(0.0);
        let y0 = mag(peak_bin.saturating_sub(1));
        let y1 = mag(peak_bin);
        let y2 = mag(peak_bin + 1);
        
        let shift = if (y0 - 2.0 * y1 + y2).abs() < 1e-10 {
            0.0
        } else {
            0.5 * (y0 - y2) / (y0 - 2.0 * y1 + y2)
        };
        
        let true_peak = peak_bin as f32 + shift;
    
    
        // Estimate sample rate based on 480 samples at 48kHz → 10ms
        let freq = (true_peak * sample_rate) / len as f32;
        let noise_threshold = match self.noise_floor_level {
            0 => 1e-5,
            1 => 1e-4,
            2 => 1e-3,
            _ => 1e-4,
        };
        
        if freq < 20.0 || freq > 5000.0 || input.iter().all(|x| x.norm() < noise_threshold) {
            return (0.0, len);
        }

        // optionally log:
        log_status(&format!(
            "smoothed_y: {:>7.4} | freq: {:>7.1} Hz | Note: {:<14} | bin est: {:>4} | bin_w: {:>11.8} || 2nd: {:>7.1} Hz ({})",
            self.smoothed_y,
            freq,
            frequency_to_note(freq),
            (freq / (48000.0 / len as f32)).round(),
            48000.0 / len as f32,
            secondary_freq,
            secondary_note
        ));
                        
        // optionally log - End.
        self.avg_freq = 0.9 * self.avg_freq + 0.1 * freq;
        self.avg_bin = 0.9 * self.avg_bin + 0.1 * true_peak;
        self.last_cli_update = Instant::now();
        
        return (self.avg_freq, len);
        
    }
    

    
}


