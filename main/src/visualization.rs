use egui_plot::{Plot, Line, PlotPoints};
use eframe::egui;
use crate::audio::AudioVisualizer;

impl eframe::App for AudioVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Real-Time Audio FFT Visualizer");

            let waveform_data = self.waveform.lock().unwrap();
            let fft_data = self.fft_result.lock().unwrap();
            let is_playing = *self.is_playing.lock().unwrap();

            Plot::new("Waveform").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate()
                        .map(|(i, &y)| [i as f64, y])
                        .collect()
                );                
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            Plot::new("FFT").show(ui, |plot_ui| {
                let fft_x_scale = (fft_data.len() as f64) / 2.0;
                let points = PlotPoints::new(
                    fft_data.iter().enumerate()
                        .map(|(i, &y)| [(i as f64) * fft_x_scale, y])
                        .collect()
                );                
                plot_ui.line(Line::new(points).name("FFT"));
            });

            if !is_playing {
                ui.label("Playback finished.");
            }
        });

        if *self.is_playing.lock().unwrap() {
            ctx.request_repaint();
        }
    }
}
