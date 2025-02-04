# Chat 04

##Request

### Question:
Is it possible to make the gui update live?

### Current Behaviour:
Gui window displays historical trace after audio has played back.

### Given the following 2 files:

Cargo.toml

```toml
[package]
name = "main"
version = "0.1.0"
edition = "2021"

[dependencies]
hound = "3.5"
rodio = "0.17"
egui_plot = "0.24"
rustfft = "6"
winapi = { version = "0.3", features = ["winuser"] }
eframe = { version = "0.24", features = ["persistence", "glow"] }
```


main.rs

```rust
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
        let filename = "./test.wav"; // Change this to your WAV file

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
    let filename = "./test.wav"; // Change this to your WAV file
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
    let filename = "./test.wav"; // Change this to your file
    
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

```

## Response

Yes, you can make the GUI update live while the audio is playing by modifying your code to:

Continuously read audio samples in a separate thread.
Update the GUI state asynchronously.
Use ctx.request_repaint() to refresh the UI.
Steps to Achieve Live Updating:
1. Refactor AudioVisualizer to Continuously Update
Modify the AudioVisualizer struct to store live audio samples and update them while the audio is playing.

2. Run Audio Processing in a Background Thread
Spawn a separate thread to read and process audio data while continuously updating the GUI.

3. Trigger UI Repaints
Use ctx.request_repaint() to ensure the GUI updates at regular intervals.

Updated Code with Live GUI Update
Modify AudioVisualizer as follows:

rust
Copy
Edit
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
Key Changes for Live Updates
Arc<Mutex<Vec<f64>> for Shared State

Enables safe concurrent modification of waveform and fft_result.
Background Thread for Audio Processing

Loads and computes FFT without blocking the UI.
Continuous UI Updates with ctx.request_repaint()

Ensures the UI refreshes while the audio is playing.
Expected Behavior
The GUI updates live while audio is playing.
The waveform and FFT refresh as the file is being processed.
No freezing or waiting until playback finishes.
This should give you a real-time audio visualization while playing back the test.wav file. 🚀












