use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use hound;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

use egui_plot::{Plot, Line, PlotPoints};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use eframe::egui;

struct AudioVisualizer {
    waveform: Arc<Mutex<Vec<f64>>>,
    fft_result: Arc<Mutex<Vec<f64>>>,
}

impl AudioVisualizer {
    fn new() -> Self {
        let waveform = Arc::new(Mutex::new(Vec::new()));
        let fft_result = Arc::new(Mutex::new(Vec::new()));

        let waveform_clone = Arc::clone(&waveform);
        let fft_result_clone = Arc::clone(&fft_result);

        // Spawn a background thread to load and process audio
        thread::spawn(move || {
            let filename = "./test.wav"; 
            let reader = hound::WavReader::open(filename).expect("Failed to open file");
            let samples: Vec<f64> = reader.into_samples::<i16>()
                .filter_map(Result::ok)
                .map(|s| s as f64)
                .collect();

            // Store waveform
            {
                let mut waveform_data = waveform_clone.lock().unwrap();
                *waveform_data = samples.clone();
            }

            // Compute FFT and store it
            {
                let mut fft_data = fft_result_clone.lock().unwrap();
                *fft_data = Self::compute_fft(&samples);
            }
        });

        Self { waveform, fft_result }
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
            ui.heading("Live Audio FFT Visualizer");

            let waveform_data = self.waveform.lock().unwrap();
            let fft_data = self.fft_result.lock().unwrap();

            // Plot waveform
            Plot::new("Waveform").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate()
                        .map(|(i, &y)| [i as f64, y])
                        .collect()
                );
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            // Plot FFT
            Plot::new("FFT").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    fft_data.iter().enumerate()
                        .map(|(i, &y)| [i as f64, y])
                        .collect()
                );
                plot_ui.line(Line::new(points).name("FFT"));
            });
        });

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
}

fn main() {
    let filename = "./test.wav";
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");

    let file = File::open(filename).expect("Failed to open file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    // Play the audio asynchronously
    let _ = stream_handle.play_raw(source.convert_samples());

    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Live Audio FFT Visualizer",
        options,
        Box::new(|_cc| Box::new(AudioVisualizer::new())),
    ) {
        eprintln!("Error running eframe: {}", e);
    };
}

