// src/pipeline/waveform_pipeline.rs

use crate::buffer::AudioBuffer;
use crate::analytics::waveform_analytics::WaveformAnalytics;
use crate::gui::waveform_gui::WaveformGui;
use crate::analytics::waveform_analytics::Waveform;


pub struct WaveformPipeline {
    analytics: WaveformAnalytics,
    gui: WaveformGui,
    recent_peaks: Vec<f32>,
}

impl WaveformPipeline {

    pub fn new() -> Self {
        Self {
            analytics: WaveformAnalytics::new(),
            gui: WaveformGui::new(),
            recent_peaks: Vec::with_capacity(100),
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

    pub fn y_range(&self) -> f32 {
        let max = self.recent_peaks.iter().copied().fold(0.0, f32::max);
        (max * 1.2).clamp(0.001, 1.0)
    }
    
    
    
}


