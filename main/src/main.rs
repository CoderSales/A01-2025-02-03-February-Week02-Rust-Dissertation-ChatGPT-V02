// load audio with hound:
use hound;

// playback audio with rodio:
use rodio::{Decoder, OutputStream, source::Source};
// use rodio::{Decoder, OutputStream}; // New
use std::fs::File;
use std::io::BufReader;

// display waveform in GUI using FFT:
// use eframe::{egui, egui::plot::{Plot, Line, Values}};
use egui_plot::{Plot, Line, PlotPoints};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

use eframe::egui;
// use egui::{CentralPanel, Context};
use egui::Context;



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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    // fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio FFT Visualizer");

            // Plot waveform
            Plot::new("Waveform").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    self.waveform.iter().enumerate()
                        .map(|(i, &y)| [i as f64, y])
                        .collect()
                );
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            // Plot FFT
            Plot::new("FFT").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    self.fft_result.iter().enumerate()
                        .map(|(i, &y)| [i as f64, y])
                        .collect()
                );
                plot_ui.line(Line::new(points).name("FFT"));
            });
        });
    }
}




fn main() {
    // 1. load audio test.wav using hound crate:
    let filename = "test.wav"; // Change this to your WAV file
    let reader = match hound::WavReader::open(filename) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            return;
        }
    };

    let spec = reader.spec();
    println!("Sample Rate: {}", spec.sample_rate);
    println!("Bits per Sample: {}", spec.bits_per_sample);
    println!("Channels: {}", spec.channels);

    let samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();
    println!("Loaded {} samples", samples.len());
    
    
    // 2. playback audio test.wav using rodio crate:
    let filename = "test.wav"; // Change this to your file
    
    // Create an audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    
    // Open and decode the audio file
    let file = File::open(filename).expect("Failed to open file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
    
    // Play the audio
    stream_handle.play_raw(source.convert_samples()).expect("Failed to play audio");
    
    // Prevents premature termination (wait for playback to complete)
    std::thread::sleep(std::time::Duration::from_secs(12)); // Adjust based on file length


    // 3. GUI with (Fast Fourier Transform) FFT Calculation

    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Audio FFT Visualizer",
        options,
        Box::new(|_cc| Box::new(AudioVisualizer::new())),
    ) {
        eprintln!("Error running eframe: {}", e);
    };

}








