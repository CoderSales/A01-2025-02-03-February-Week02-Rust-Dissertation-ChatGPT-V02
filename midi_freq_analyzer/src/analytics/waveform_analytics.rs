// src/analytics/waveform_analytics.rs

use crate::buffer::AudioBuffer;

#[derive(Default)]
pub struct Waveform {
    pub samples: Vec<f32>,
}

pub struct WaveformAnalytics;

impl WaveformAnalytics {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, buffer: &AudioBuffer) -> Waveform {
        Waveform {
            samples: buffer.samples.clone(),
        }
    }
}
