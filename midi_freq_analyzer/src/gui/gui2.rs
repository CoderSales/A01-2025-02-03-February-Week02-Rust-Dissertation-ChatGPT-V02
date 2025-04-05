// src/gui/gui2.rs

use eframe::egui::{self, Color32, Ui, ProgressBar};
use crate::analytics::frequency_analytics::FrequencyBands;

pub struct FrequencyGui;

impl FrequencyGui {
    pub fn new() -> Self {
        Self
    }

    pub fn show_bars(&self, ui: &mut Ui, bands: &FrequencyBands) {
        let items = [
            ("Low", bands.low),
            ("Mid", bands.mid),
            ("High", bands.high),
        ];

        for (label, val) in items {
            let norm = val.clamp(0.0, 1.0);
            ui.label(label);
            ui.add(
                ProgressBar::new(norm)
                    .desired_width(100.0)
                    .fill(Color32::LIGHT_BLUE)
                    .text(format!("{:.1} dB", norm * 100.0 - 100.0)),
            );
            ui.separator();
        }
    }
}
