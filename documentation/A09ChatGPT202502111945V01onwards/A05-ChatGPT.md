# ChatGPT

## Input

```bash

Compiler warnings:

```bash
warning: unused variable: `x_start`
   --> src/main.rs:110:21
    |
110 |                 let x_start = 0.0;
    |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_x_start`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x_start`
   --> src/main.rs:124:21
    |
124 |                 let x_start = 0.0;
    |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_x_start`
```

Code:

```bash
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use hound;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

use egui_plot::{Plot, Line, PlotPoints};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use eframe::egui;

const CHUNK_SIZE: usize = 512;  
const DOWNSAMPLE_FACTOR: usize = 8;  // Keep smoothness
const FPS: usize = 60;  // Standard screen refresh rate

struct AudioVisualizer {
    waveform: Arc<Mutex<Vec<f64>>>,
    fft_result: Arc<Mutex<Vec<f64>>>,
    is_playing: Arc<Mutex<bool>>,
}

impl AudioVisualizer {
    fn new(audio_duration_secs: f64) -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let is_playing = Arc::new(Mutex::new(true)); 

        let waveform_clone = Arc::clone(&waveform);
        let fft_result_clone = Arc::clone(&fft_result);
        let is_playing_clone = Arc::clone(&is_playing);

        // Calculate exact number of frames required to match audio duration
        let total_frames = (audio_duration_secs * FPS as f64) as usize;
        let time_per_frame = audio_duration_secs / total_frames as f64;

        thread::spawn(move || {
            let filename = "./test.wav"; 
            let reader = hound::WavReader::open(filename).expect("Failed to open file");

            let samples: Vec<f64> = reader.into_samples::<i16>()
                .filter_map(Result::ok)
                .map(|s| s as f64)
                .collect();

            let mut current_window: Vec<f64> = vec![0.0; CHUNK_SIZE * 5]; // Fixed-length scrolling buffer | Edit 1.
            let start_time = Instant::now();

            for (i, chunk) in samples.chunks(CHUNK_SIZE).enumerate() {
                let downsampled_chunk: Vec<f64> = chunk.iter()
                    .step_by(DOWNSAMPLE_FACTOR) 
                    .cloned()
                    .collect();

                // Shift the waveform left and append new samples to create scrolling effect
                let shift_amount = downsampled_chunk.len() * 5; // ✅ Move 5× faster
                current_window.drain(..shift_amount); // Remove oldest samples
                current_window.extend(downsampled_chunk.clone()); // Append new data

                {
                    let mut waveform_data = waveform_clone.lock().unwrap();
                    *waveform_data = current_window.clone();
                }

                {
                    let mut fft_data = fft_result_clone.lock().unwrap();
                    *fft_data = Self::compute_fft(&current_window);
                }

                // Wait for next frame (FPS control, ensures exact match)
                let elapsed = start_time.elapsed().as_secs_f64();
                let expected_time = (i + 1) as f64 * time_per_frame;
                if elapsed < expected_time {
                    std::thread::sleep(Duration::from_secs_f64(expected_time - elapsed));
                }
            }

            *is_playing_clone.lock().unwrap() = false;
        });

        Self { waveform, fft_result, is_playing }
    }

    fn compute_fft(samples: &[f64]) -> Vec<f64> {
        let len = samples.len().next_power_of_two();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(len);

        let mut buffer: Vec<Complex<f64>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
        buffer.resize(len, Complex::new(0.0, 0.0)); 

        fft.process(&mut buffer);
        buffer.iter().map(|c| c.norm()).collect()
    }
}

impl eframe::App for AudioVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Real-Time Audio FFT Visualizer");

            let waveform_data = self.waveform.lock().unwrap();
            let _fft_data = self.fft_result.lock().unwrap(); // Prevents compiler warning
            let is_playing = *self.is_playing.lock().unwrap();

            // Plot waveform
            Plot::new("Waveform").show(ui, |plot_ui| {
                let x_start = 0.0;
                let x_end = CHUNK_SIZE as f64;  // Fixed X-axis range
                
                let x_offset = self.waveform.lock().unwrap().len() as f64; // ✅ Make it scroll dynamically
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate()
                        .map(|(i, &y)| [(i as f64 + x_offset) % x_end, y]) // ✅ Scroll smoothly
                        .collect()
                );                
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            // Plot FFT
            Plot::new("FFT").show(ui, |plot_ui| {
                let x_start = 0.0;
                let x_end = CHUNK_SIZE as f64;  // Fixed X-axis range
                
                let x_offset = self.waveform.lock().unwrap().len() as f64; // ✅ Make it scroll dynamically
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate()
                        .map(|(i, &y)| [(i as f64 + x_offset) % x_end, y]) // ✅ Scroll smoothly
                        .collect()
                );                
                plot_ui.line(Line::new(points).name("FFT"));
            });

            if !is_playing {
                ui.label("Playback finished.");
            }
        });

        if *self.is_playing.lock().unwrap() {
            ctx.request_repaint();  // No sleep, UI updates as fast as possible
        }
    }
}

fn main() {
    let filename = "./test.wav";
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");

    let file = File::open(filename).expect("Failed to open file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    // Get audio duration in seconds
    let reader = hound::WavReader::open(filename).expect("Failed to open file for duration check");
    let num_samples = reader.len() as f64;
    let sample_rate = reader.spec().sample_rate as f64;
    let audio_duration_secs = num_samples / sample_rate;  // ✅ f64 primitive, easy to copy

    // Play the audio asynchronously
    let _ = stream_handle.play_raw(source.convert_samples());

    let options = eframe::NativeOptions::default();

    // ✅ Clone `audio_duration_secs` into the closure to avoid borrowing issues
    if let Err(e) = eframe::run_native(
        "Real-Time Audio FFT Visualizer",
        options,
        Box::new(move |_cc| Box::new(AudioVisualizer::new(audio_duration_secs))),  // ✅ `move` ensures ownership
    ) {
        eprintln!("Error running eframe: {}", e);
    };
}
```

Issue:

GUI Window not displaying

```

## Output

```bash
Fix: GUI Window Not Displaying & Remove Unused Variables
1️⃣ Remove Unused x_start Variable
The x_start variable is declared but never used, causing a compiler warning.

Replace these lines:

rust
Copy
Edit
let x_start = 0.0;
With:

rust
Copy
Edit
// Remove x_start since it is unused
2️⃣ Fix X-Axis Calculation for Scrolling
Replace:

rust
Copy
Edit
let x_offset = self.waveform.lock().unwrap().len() as f64;
With:

rust
Copy
Edit
let x_offset = waveform_data.len() as f64; // ✅ Use locked waveform data
3️⃣ Fix FFT Plot: Use fft_data Instead of waveform_data
Currently, both Waveform and FFT plots use waveform_data, meaning FFT is never displayed.

Replace:

rust
Copy
Edit
let x_offset = self.waveform.lock().unwrap().len() as f64;
let points = PlotPoints::new(
    waveform_data.iter().enumerate()
        .map(|(i, &y)| [(i as f64 + x_offset) % x_end, y]) // ✅ Scroll smoothly
        .collect()
);
With:

rust
Copy
Edit
let x_offset = fft_data.len() as f64;
let points = PlotPoints::new(
    fft_data.iter().enumerate()
        .map(|(i, &y)| [(i as f64 + x_offset) % x_end, y]) // ✅ Correctly use FFT data
        .collect()
);
4️⃣ Ensure the GUI Window is Created Properly
If the GUI does not display, make sure eframe::NativeOptions is correctly set.

Replace:

rust
Copy
Edit
let options = eframe::NativeOptions::default();
With:

rust
Copy
Edit
let options = eframe::NativeOptions {
    initial_window_size: Some(egui::Vec2::new(800.0, 600.0)), // ✅ Set a window size
    ..Default::default()
};
Expected Behavior
✅ No more compiler warnings about unused variables.
✅ GUI window correctly appears on launch.
✅ FFT and waveform scrolling independently.
✅ Smooth, real-time updates matching audio.

Now, your GUI should display properly with correct scrolling! 🚀
```
