// src/pipeline/waveform_pipeline.rs

use crate::buffer::AudioBuffer;
use crate::analytics::waveform_analytics::WaveformAnalytics;
use crate::gui::waveform_gui::WaveformGui;
use crate::analytics::waveform_analytics::Waveform;
use std::time::{Instant, Duration};



pub struct WaveformPipeline {
    analytics: WaveformAnalytics,
    gui: WaveformGui,
    recent_peaks: Vec<f32>,
    last_y_update: Instant,
    smoothed_y: f32,
}

impl WaveformPipeline {

    pub fn new() -> Self {
        Self {
            analytics: WaveformAnalytics::new(),
            gui: WaveformGui::new(),
            recent_peaks: Vec::with_capacity(100),
            last_y_update: Instant::now(),
            smoothed_y: 0.01,
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
            
    
    
}


