// src/gui/waveform_gui.rs

use eframe::egui::{self, Color32, Ui};
use crate::analytics::waveform_analytics::Waveform;
use egui_plot::{Plot, Line, PlotPoints};
use crate::analytics::note_label::frequency_to_note;
use crate::pipeline::waveform_pipeline::WaveformPipeline;
use crate::buffer::AudioBuffer;



pub struct WaveformGui;

impl WaveformGui {
    pub fn new() -> Self {
        Self
    }

    pub fn display(&self, waveform: &Waveform) {
        // Placeholder: Hook into egui drawing context elsewhere
        }

        pub fn show_plot(
            &self,
            ui: &mut Ui,
            waveform: &Waveform,
            buffer: &AudioBuffer,
            pipeline: &WaveformPipeline,
        ) {        
        let points: PlotPoints = waveform
            .samples
            .iter()
            .enumerate()
            .map(|(i, &y)| [i as f64, y as f64])
            .collect();

        let line = Line::new(points).color(Color32::RED);
        let y = pipeline.y_range();
        Plot::new("Waveform")
        .include_y(-y)
        .include_y(y)
        .include_x(0.0)
        .include_x(500.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
        let freq = WaveformPipeline::new().latest_peak(buffer);
        let note_text = frequency_to_note(freq);
        ui.label(note_text);        

    }
}
