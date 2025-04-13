// src/pipeline/waveform_pipeline.rs

use crate::buffer::AudioBuffer;
use crate::analytics::waveform_analytics::WaveformAnalytics;
use crate::gui::waveform_gui::WaveformGui;
use crate::analytics::waveform_analytics::Waveform;
use std::time::{Instant, Duration};
use rustfft::{FftPlanner, num_complex::Complex};
use std::io::{self, stdout, Write};
use crate::analytics::note_label::frequency_to_note; // frequency_to_note
use crate::cli_log::log_status; // log_status 
use cpal::traits::{HostTrait, DeviceTrait};
use std::collections::{BTreeMap, HashMap, HashSet};


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
    chord_detected: bool,
    note_history: Vec<String>, // add in struct
    confirmed_notes: Vec<String>,

}

#[derive(Clone)]
struct BinActivity {
    start_hz: f32,
    end_hz: f32,
    energy: f32,
    active: bool,
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
            chord_detected: false,
            note_history: Vec::with_capacity(5),
            confirmed_notes: Vec::new(),


        }
    }

    pub fn update(&mut self, buffer: &AudioBuffer) {
        let waveform = self.analytics.process(buffer);
        self.gui.display(&waveform);
        self.detect_chord(buffer); // ← lightweight chord sweep
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

    pub fn detect_chord(&mut self, buffer: &AudioBuffer) {
        let bins = self.low_res_sweep(buffer);
        let focused_bins = Self::top_25_percent_bins(&bins);
    
        for b in &focused_bins {
            if b.energy > 0.001 {
                log_status(&format!(
                    "🎯 {:.0}-{:.0} Hz | amp {:.3}",
                    b.start_hz, b.end_hz, b.energy.abs()
                ));
            }
        }
                
    
        // ⏳ placeholder: high_res_zoom(buffer, &focused_bins);
        self.high_res_zoom(buffer, &focused_bins);
        self.final_confirm_sweep(buffer);
        self.interactive_scan_step1(buffer); // <-- Add here // Step 1.

        // overlapping bins amp probabilistic step wise 2.0
        let prob_bins = self.probabilistic_overlap_scan(buffer);
        let mut sorted: Vec<_> = prob_bins.iter().filter(|(_, &v)| v > 0.2).collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        print!("\r");
        for (label, amp) in sorted.iter().take(6) {
            print!("{} {:.2}  ", label, amp);
        }
        stdout().flush().unwrap();
        println!("\n⏸ Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
        // overlapping bins amp probabilistic step wise 2.0
        
        let found = self.guided_frequency_discovery(buffer);

        // rerun zoom from previous results - Start:
        let found = self.guided_frequency_discovery(buffer);
        self.rerun_zoom_from_previous_results(buffer, &found);
        // rerun zoom from previous results - End.
        

        println!("📌 Confirmed freqs: {:?}", found);


    }
    

    // sweep three:
    pub fn final_confirm_sweep(&mut self, buffer: &AudioBuffer) {
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 512;
        let sample_rate = 48000.0;
    
        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);
    
        let bin_width = sample_rate / len as f32;
        let start = (250.0 / bin_width) as usize;
        let end = (400.0 / bin_width) as usize;
    
        let peak_bin = input[start..end]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.norm().partial_cmp(&b.1.norm()).unwrap())
            .map(|(i, _)| i + start)
            .unwrap_or(0);

    
        let freq = peak_bin as f32 * bin_width;
        let note = frequency_to_note(freq);
        let rounded = (freq * 1000.0).round() / 1000.0;
        let tolerance = 0.5; // Hz
        log_status(&format!(
            "🎯 Final sweep {:.1}–{:.1} Hz → {:.3} Hz ±{:.1} Hz = {}",
            freq - tolerance,
            freq + tolerance,
            rounded,
            tolerance,
            note
        ));
                
        let note_str = Self::base_note_name(&note).to_string();
        self.note_history.push(note_str.clone());
        self.confirmed_notes.push(note_str.clone());
        if self.note_history.len() > 7 { self.note_history.remove(0); }


        // iterative zoom scan: Start:
        if let Some((freq, note)) = self.iterative_zoom_scan(buffer, (240.0, 400.0)) {
            let note_str = note.clone();
        
            let mut temp = self.note_history.clone();
            temp.push(note_str.clone());
            let uniq: HashSet<&str> = temp.iter().map(|s| s.as_str()).collect();
        
        
            self.note_history.push(note_str);
            if self.note_history.len() > 7 {
                self.note_history.remove(0);
            }
        
            log_status(&format!("🧪 note_history = {:?}", self.note_history));
            log_status(&format!("🎯 Final sweep 250–400 Hz → {:.3} Hz = {}", freq, note));
        }        
        // iterative zoom scan: End.


        log_status(&format!("🧪 note_history = {:?}", self.note_history));
        log_status(&format!("🎯 Final sweep 250–400 Hz → {:.1} Hz = {}", freq, note));

    }
    
    // sweep three end.

    // prompt driven cli phased frequency scan

    pub fn interactive_scan_step1(&mut self, buffer: &AudioBuffer) {
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 64;
        let sample_rate = 48000.0;
    
        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);
    
        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
        let bin_width = sample_rate / len as f32;
    
        let ranges = (0..10).map(|i| (100.0 + i as f32 * 40.0, 100.0 + (i + 1) as f32 * 40.0));
    
        print!("\r");
        for (start_hz, end_hz) in ranges {
            let start = (start_hz / bin_width) as usize;
            let end = (end_hz / bin_width) as usize;
            let amp: f32 = magnitudes[start..end].iter().copied().fold(0.0, f32::max);
            if amp >= 0.01 {
                print!("{:.0}-{:.0} {:.2}  ", start_hz, end_hz, amp);
            }            
        }
        io::stdout().flush().unwrap();
    
        // Pause
        println!("\n⏸ Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
    

    // prompt driven cli phased frequency scan- End.

    // overlapping bins amp probabilistic step wise 2.0

    pub fn probabilistic_overlap_scan(&mut self, buffer: &AudioBuffer) -> HashMap<String, f32> {
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 64;
        let sample_rate = 48000.0;

        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);

        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
        let bin_width = sample_rate / len as f32;

        let mut results = HashMap::new();
        let step_hz = 50.0;
        let window_hz = 100.0;
        let start_hz = 100.0;
        let end_hz = 500.0;

        let mut cursor = start_hz;
        while cursor + window_hz <= end_hz {
            let start_idx = (cursor / bin_width) as usize;
            let end_idx = ((cursor + window_hz) / bin_width) as usize;

            let max_amp = magnitudes[start_idx..end_idx]
                .iter()
                .copied()
                .fold(0.0, f32::max);

            let label = format!("{}_{:.0}", cursor as usize, cursor + window_hz);
            results.insert(label, max_amp);

            cursor += step_hz;
        }

        results
    }


    // overlapping bins amp probabilistic step wise 2.0 - End.

    // user guided frequency zoom - Start 3.0
    pub fn guided_frequency_discovery(&mut self, buffer: &AudioBuffer) -> BTreeMap<String, f32> {
        let mut confirmed_freqs = BTreeMap::new();
    
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 64;
        let sample_rate = 48000.0;
    
        let mut input: Vec<Complex<f32>> = samples
            .iter().cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);
    
        let bin_width = sample_rate / len as f32;
        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
    
        let bands = 20;
        let min_hz = 5.0;
        let max_hz = 22000.0;
        let step = (max_hz as f32 / min_hz).powf(1.0 / bands as f32);
        let mut region = 1;
    
        let mut lo = min_hz;
        for _ in 0..bands {
            let hi = lo * step;
            let start = (lo / bin_width) as usize;
            let end = (hi / bin_width).min(len as f32 / 2.0) as usize;
    
            let max_amp = magnitudes[start..end].iter().copied().fold(0.0, f32::max);
            if max_amp > 0.001 {
                let mut best_freq = 0.0;
                let mut best_amp = 0.0;
                let steps = 10;
                for s in 0..steps {
                    let sub_lo = lo + (hi - lo) * (s as f32 / steps as f32);
                    let sub_hi = lo + (hi - lo) * ((s + 1) as f32 / steps as f32);
                    let sub_start = (sub_lo / bin_width) as usize;
                    let sub_end = (sub_hi / bin_width) as usize;
                    let amp = magnitudes[sub_start..sub_end].iter().copied().fold(0.0, f32::max);
                    if amp > best_amp {
                        best_amp = amp;
                        best_freq = (sub_lo + sub_hi) / 2.0;
                    }
                }
                if best_amp > 0.1 {
                    let label = format!("{}st", region);
                    confirmed_freqs.insert(label, best_freq);
                    region += 1;
                }
            }
            lo = hi;
        }
    
        confirmed_freqs
    }
    

    // user guided frequency zoom - End  3.0

    // Hanning window to reduce spectral leakage during FFT - 3.1 - Start:
    fn apply_hanning(input: &mut [Complex<f32>]) {
        let n = input.len();
        for (i, val) in input.iter_mut().enumerate() {
            let w = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n as f32 - 1.0)).cos());
            val.re *= w;
        }
    }    
    // Hanning window to reduce spectral leakage during FFT - 3.1 - End.

    // make confirmed_notes public - Start:
    pub fn confirmed_notes(&self) -> &[String] {
        &self.confirmed_notes
    }
    // make confirmed_notes public - End.

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
    
    
    pub fn low_res_sweep(&mut self, buffer: &AudioBuffer) -> Vec<BinActivity> {
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 64;
        let sample_rate = 48000.0;
    
        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);
            
        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
        let bin_width = sample_rate / len as f32;
        let bin_group_hz = 5000.0 / 100.0;
    
        let mut bins = Vec::with_capacity(100);
        for i in 0..100 {
            let start = (i as f32 * bin_group_hz / bin_width) as usize;
            let end = ((i + 1) as f32 * bin_group_hz / bin_width) as usize;
    
            let energy: f32 = magnitudes[start..end.min(magnitudes.len())]
                .iter()
                .copied()
                .sum::<f32>()
                / (end - start).max(1) as f32;
    
            bins.push(BinActivity {
                start_hz: i as f32 * bin_group_hz,
                end_hz: (i + 1) as f32 * bin_group_hz,
                energy,
                active: energy > 0.01,
            });
        }
    
        bins
    }
    
    fn top_25_percent_bins(bins: &[BinActivity]) -> Vec<BinActivity> {
        let mut sorted = bins.to_vec();
        sorted.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
    
        let top_n = (sorted.len() as f32 * 0.25).ceil() as usize;
        sorted.into_iter().take(top_n).collect()
    }


    fn high_res_zoom(&mut self, buffer: &AudioBuffer, bins: &[BinActivity]) {
        let samples = &buffer.samples;
        let len = samples.len().next_power_of_two() * 256;
        let sample_rate = 48000.0;
    
        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
        fft.process(&mut input);
            
        let bin_width = sample_rate / len as f32;
    
        for b in bins {
            let start = (b.start_hz / bin_width) as usize;
            let end = (b.end_hz / bin_width) as usize;
    
            let (peak_bin, peak_mag) = input[start..end]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.norm().partial_cmp(&b.1.norm()).unwrap())
                .map(|(i, c)| (i + start, c.norm()))
                .unwrap_or((0, 0.0));
    
            let freq = peak_bin as f32 * bin_width;
            if peak_mag >= 0.01 {
                let note = frequency_to_note(freq);
                let note_label = Self::base_note_name(&note);
                if note_label != "---" {
                    let prob = (peak_mag * 50.0).clamp(1.0, 100.0).round();
                    log_status(&format!(
                        "🔍 Zoomed bin {:.0}-{:.0} Hz → {:.1} Hz = {}  amp={:.2} prob={:.0}%",
                        b.start_hz, b.end_hz, freq, note, peak_mag, prob
                    ));
                }
            }
        }
    }
    
    // iterative zoom scan: Start:
    fn iterative_zoom_scan(&self, buffer: &AudioBuffer, range: (f32, f32)) -> Option<(f32, String)> {
        let (mut lo, mut hi) = range;
        let sample_rate = 48000.0;
        let mut peak_freq = 0.0;
        let mut best_mag = 0.0;

        for _ in 0..5 {
            let len = buffer.samples.len().next_power_of_two() * 512;
            let mut input: Vec<Complex<f32>> = buffer.samples
                .iter().cloned()
                .map(|x| Complex { re: x, im: 0.0 })
                .collect();
            input.resize(len, Complex { re: 0.0, im: 0.0 });

            Self::apply_hanning(&mut input);
            let mut planner = FftPlanner::<f32>::new();
            let fft = planner.plan_fft_forward(len);
            fft.process(&mut input);

            let bin_width = sample_rate / len as f32;
            let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();

            let start = (lo / bin_width) as usize;
            let end = (hi / bin_width).min((len / 2) as f32) as usize;

            if let Some((i, &mag)) = magnitudes[start..end]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                peak_freq = (start + i) as f32 * bin_width;
                best_mag = mag;
                lo = peak_freq - bin_width;
                hi = peak_freq + bin_width;
            } else {
                break;
            }
        }

        if best_mag > 0.01 {
            let note = frequency_to_note(peak_freq);
            let label = Self::base_note_name(&note).to_string();
            println!(
                "🎯 Final sweep {:.1}–{:.1} Hz → {:.3} Hz ±0.5 Hz = {} ({:.1} Hz)",
                lo,
                hi,
                peak_freq,
                label,
                peak_freq
            );

            println!("Add {} to note history? (y/n)", label);
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            if input.trim().to_lowercase() == "y" {
                Some((peak_freq, label))
            } else {
                None
            }
        } else {
            None
        }
    }    
    // iterative zoom scan: End.


    // rerun zoom from previous results - Start:
    pub fn rerun_zoom_from_previous_results(&mut self, buffer: &AudioBuffer, previous: &BTreeMap<String, f32>) {
        for (label, center_freq) in previous {
            let lo = center_freq - 50.0;
            let hi = center_freq + 50.0;
    
            if let Some((freq, note)) = self.iterative_zoom_scan(buffer, (lo, hi)) {
                let note_str = note.clone();
            
                if self.note_history.len() >= 7 {
                    self.note_history.remove(0);
                }
                self.note_history.push(note_str.clone());
                self.confirmed_notes.push(note_str.clone());
            
                log_status(&format!("🔁 Rescan {} → {:.3} Hz = {}", label, freq, note));
            }
        }    
    }
    // rerun zoom from previous results - End.

    pub fn dominant_frequency(&mut self, buffer: &AudioBuffer) -> (f32, usize) {
        Self::list_input_sample_rates(); // print available rates

        let samples = &buffer.samples;
        if samples.iter().all(|&x| x.abs() < 1e-6) {
            return (0.0, 0);
        }
    
        // let len = samples.len().next_power_of_two();
        // let len = 16_384;
        let len = samples.len().next_power_of_two() * 64 * 2;

        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|x| Complex { re: x, im: 0.0 })
            .collect();
        input.resize(len, Complex { re: 0.0, im: 0.0 });
    
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(len);
        Self::apply_hanning(&mut input);
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


        let sample_rate = 48000.0;

        let bin_w = sample_rate / len as f32;

        
        let top_n = 150;
        let mut raw_peaks: Vec<(usize, f32)> = magnitudes
            .iter()
            .copied()
            .enumerate()
            .take(len / 2)
            .collect();
        
        raw_peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let mut top_peaks = Vec::new();
        for (i, mag) in raw_peaks {
            let freq = i as f32 * bin_w;
            if !top_peaks.iter().any(|(j, _)| ((j * bin_w as usize) as f32 - freq).abs() < 30.0) {
                top_peaks.push((i, mag));
                if top_peaks.len() >= 5 {
                    break;
                }
            }
        }
        
        // replace lines 153 to 186:
        // Convert top bins to base note names
        


                
        // end of most of replacement of lines 153 to 186:
    
        



        let min_gap_hz = 40.0;
        let primary_freq: f32 = primary as f32 * bin_w;

        let mags: Vec<f32> = magnitudes
        .iter()
        .enumerate()
        .map(|(i, &m)| {
            let f = i as f32 * bin_w;
            m // No suppression — just pass through the magnitude
        })
        .collect();

        


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


        let secondary_freq = (secondary_bin * sample_rate) / len as f32;
        let secondary_note = frequency_to_note(secondary_freq);

        // convert secondary peak bin to Hz and note name - End.

        // third frequency Start:
        // let primary_freq = primary as f32 * bin_w;
        let secondary_freq_est = secondary as f32 * bin_w;

        let mags3: Vec<f32> = magnitudes
            .iter()
            .enumerate()
            .map(|(i, &m)| {
                let f = i as f32 * bin_w;
                if (f - primary_freq).abs() < min_gap_hz || (f - secondary_freq_est).abs() < min_gap_hz {
                    0.0
                } else {
                    m
                }
            })
            .collect();
        
        let third = mags3
            .iter()
            .take(len / 2)
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let third_freq = third as f32 * bin_w;
        let third_note = frequency_to_note(third_freq);
        

        // third frequency End.

        let primary_note = frequency_to_note(primary_freq);
        let secondary_note = frequency_to_note(secondary_freq);
        let third_note = frequency_to_note(third_freq);
        
        let just_notes = vec![
            Self::base_note_name(&primary_note).to_string(),
            Self::base_note_name(&secondary_note).to_string(),
            Self::base_note_name(&third_note).to_string(),
        ];
        
        


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


        let primary_mag = magnitudes[primary];
        let secondary_mag = magnitudes[secondary];
        let third_mag = magnitudes[third];

        let mag_thresh = 0.01;
        if primary_mag < mag_thresh || secondary_mag < mag_thresh || third_mag < mag_thresh {
            return (0.0, len);
        }

        // line 303.            
        // // Chord detection


        let unique_notes: HashSet<String> = just_notes.clone().into_iter().collect();
        let note_refs: Vec<&str> = unique_notes.iter().map(|s| s.as_str()).collect();
        
        if note_refs.contains(&"C") && note_refs.contains(&"E") && note_refs.contains(&"G") {
            log_status("🔔 Detected: C + E + G");
        }
        
        
        // let chord = if Self::matches_note("C", &note_refs)
        // && Self::matches_note("E", &note_refs)
        // && Self::matches_note("G", &note_refs)
        // {            
        //     "C Major"
        // } else {
        //     "---"
        // };

        let chord = if Self::matches_note("C", &note_refs)
        && Self::matches_note("E", &note_refs)
        && Self::matches_note("G", &note_refs)
        {
            "C Major"
        } else {
            "---"
        };
    

        let now = Instant::now();
        if now.duration_since(self.last_cli_update) >= Duration::from_millis(200) {
            self.last_cli_update = now;

            log_status(&format!(
                "Primary: {:.1} Hz, Secondary: {:.1} Hz, Third: {:.1} Hz || Notes: {:?}
                ...|| Chord: {:<8} || Notes: {:?}",
                primary_freq, secondary_freq, third_freq, just_notes,
                chord, note_refs
            ));
                        
        }
                
        // optionally log - End.
        self.avg_freq = 0.9 * self.avg_freq + 0.1 * freq;
        self.avg_bin = 0.9 * self.avg_bin + 0.1 * true_peak;
        // self.last_cli_update = Instant::now();
        
        return (self.avg_freq, len);
        
    }
    
    fn base_note_name(s: &str) -> &str {
        if s.len() >= 2 && s.chars().nth(1).unwrap() == '#' {
            &s[..2] // Keep sharp: e.g. "C#" from "C#4"
        } else if s.len() >= 1 {
            &s[..1] // e.g. "C" from "C4"
        } else {
            "---"
        }
    }
    
    
    
    pub fn list_input_sample_rates() {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No input device available");
    
        let config_range = device
            .supported_input_configs()
            .expect("Error querying configs");
    
        for config in config_range {
            // println!(
            //     "Supported: {:?} - {:?} Hz @ {:?}",
            //     config.min_sample_rate().0,
            //     config.max_sample_rate().0,
            //     config.sample_format()
            // );
        }
    }
    
    pub fn matches_note(note: &str, targets: &[&str]) -> bool {
        targets.iter().any(|t| t == &note)
    }
    
    
}

