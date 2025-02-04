use eframe::{egui, egui::plot::{Plot, Line, Values}};
use rodio::{Decoder, OutputStream};
use hound;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use std::fs::File;
use std::io::BufReader;

struct AudioVisualizer {
    waveform: Vec<f64>,
    fft_result: Vec<f64>,
}

impl AudioVisualizer {
    fn new() -> Self {
        let filename = "example.wav"; // Change this to your WAV file

        // Load WAV file
        let reader = hound::WavReader::open(filename).expect("Failed to open file");
        let samples: Vec<f64> = reader
            .into_samples::<i16>()
            .filter_map(Result::ok)
            .map(|s| s as f64)
            .collect();

        // Compute FFT
        let fft_result = Self::compute_fft(&samples);

        Self { waveform: samples, fft_result }
    }

    fn compute_fft(samples: &[f64]) -> Vec<f64> {
        let len = samples.len().next_power_of_two();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(len);

        let mut buffer: Vec<Complex<f64>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
        buffer.resize(len, Complex::new(0.0, 0.0)); // Zero-padding

        fft.process(&mut buffer);
        buffer.iter().map(|c| c.norm()).collect()
    }
}

impl eframe::App for AudioVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio FFT Visualizer");

            // Plot waveform
            Plot::new("Waveform").show(ui, |plot_ui| {
                let points: Values = self.waveform.iter().enumerate()
                    .map(|(i, &y)| [i as f64, y])
                    .collect::<Vec<_>>().into();
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            // Plot FFT
            Plot::new("FFT").show(ui, |plot_ui| {
                let points: Values = self.fft_result.iter().enumerate()
                    .map(|(i, &y)| [i as f64, y])
                    .collect::<Vec<_>>().into();
                plot_ui.line(Line::new(points).name("FFT"));
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Audio FFT Visualizer",
        options,
        Box::new(|_cc| Box::new(AudioVisualizer::new())),
    );
}
