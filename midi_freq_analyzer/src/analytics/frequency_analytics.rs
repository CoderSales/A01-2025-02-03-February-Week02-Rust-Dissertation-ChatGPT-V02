// src/analytics/frequency_analytics.rs

use crate::buffer::AudioBuffer;

#[derive(Default, Clone, Copy)]
pub struct FrequencyBands {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
}

pub struct FrequencyAnalytics;

impl FrequencyAnalytics {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, buffer: &AudioBuffer) -> FrequencyBands {
        let sum: f32 = buffer.samples.iter().sum();
        let avg = sum / buffer.samples.len().max(1) as f32;

        FrequencyBands {
            low: avg * 0.6,
            mid: avg,
            high: avg * 1.4,
        }
    }
}
