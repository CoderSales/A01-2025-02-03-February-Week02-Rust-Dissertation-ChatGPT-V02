// src/gui/waveform_gui.rs

use eframe::egui::{self, Color32, Ui};
use crate::analytics::waveform_analytics::Waveform;
use egui_plot::{Plot, Line, PlotPoints};


pub struct WaveformGui;

impl WaveformGui {
    pub fn new() -> Self {
        Self
    }

    pub fn display(&self, waveform: &Waveform) {
        // Placeholder: Hook into egui drawing context elsewhere
        println!("Waveform length: {}", waveform.samples.len());
    }

    pub fn show_plot(&self, ui: &mut Ui, waveform: &Waveform) {

        let points: PlotPoints = waveform
            .samples
            .iter()
            .enumerate()
            .map(|(i, &y)| [i as f64, y as f64])
            .collect();

        let line = Line::new(points).color(Color32::RED);
        Plot::new("Waveform").show(ui, |plot_ui| {
            plot_ui.line(line);
        });
    }
}
