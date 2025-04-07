// src/pipeline/frequency_pipeline.rs

use crate::buffer::AudioBuffer;
use crate::analytics::frequency_analytics::{FrequencyAnalytics, FrequencyBands};
use crate::gui::gui2::FrequencyGui;

pub struct FrequencyPipeline {
    analytics: FrequencyAnalytics,
    gui: FrequencyGui,
}

impl FrequencyPipeline {
    pub fn new() -> Self {
        Self {
            analytics: FrequencyAnalytics::new(),
            gui: FrequencyGui::new(),
        }
    }

    pub fn update(&mut self, buffer: &AudioBuffer) {
        let bands: FrequencyBands = self.analytics.process(buffer);
        // GUI drawing expected in UI context externally
        // println!("Freq Bands: L {:.2}, M {:.2}, H {:.2}", bands.low, bands.mid, bands.high);
    }

    pub fn show(&self, ui: &mut egui::Ui, buffer: &AudioBuffer) {
        let bands = self.analytics.process(buffer);
        self.gui.show_bars(ui, &bands);
    }
}
