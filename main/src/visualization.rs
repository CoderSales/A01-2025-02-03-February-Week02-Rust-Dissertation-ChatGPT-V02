use crate::audio::AudioProcessor;
use eframe::egui::{self, CentralPanel, Button};
use egui_plot::{Plot, Line, PlotPoints};

pub struct Visualization {
    audio: AudioProcessor,
    is_listening: bool,  // ✅ Add listening state
}

impl Visualization {
    pub fn new() -> Self {
        Self { 
            audio: AudioProcessor::new(),
            is_listening: false,  // ✅ Default is not listening
        }
    }
}

impl eframe::App for Visualization {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Audio Visualization");

            if ui.button("🎤 Listen").clicked() {
                self.is_listening = true;
            }

            if ui.button("🛑 Stop Listening").clicked() {
                self.is_listening = false;
            }

            let waveform_data = self.audio.waveform.lock().unwrap();
            let fft_data = self.audio.fft_result.lock().unwrap();
            let dominant_freq = *self.audio.dominant_frequency.lock().unwrap();

            Plot::new("Waveform").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                );
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            Plot::new("FFT").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    fft_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                );
                plot_ui.line(Line::new(points).name("FFT"));
            });

            ui.label(format!("Dominant Frequency: {:.2} Hz", dominant_freq));
            ui.label(format!("Chord: {}", Visualization::detect_chord(dominant_freq)));
        });

        ctx.request_repaint();
    }
}
