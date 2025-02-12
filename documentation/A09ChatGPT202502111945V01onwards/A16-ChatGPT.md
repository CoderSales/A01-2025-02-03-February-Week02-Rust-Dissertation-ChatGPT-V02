# Chat

## Input

This Chat request is a stretch objective.

### Notes on

#### Visual Scaling Aspect ratio

0.09 y axis height

to

130 x axis width

#### Audio to Video Playback ratio of speed

visual finishes slightly ahead of audio, but almost perfect.

### Suggestion

Possibly if video could playback at about 11 / 12ths of current speed
this might sync up audio and video closer.

### Next features

#### Frequency analysis

Could it be possible to listen for frequencies and output them?

i.e.

#### Recording

Record live audio in and play back output video close to live

#### Instrument Chord detection

If playing a guitar or piano chord

the software would:

- detect the main frequencies

- estimate the chord being played e.g. C Major

#### package code

Package parts of code into separate files

#### Frequency display

Show frequencies on a scale from 5 Hz to 20 kHz

#### Take input from user

Create GUI input slider for volume

#### Sync processing suggestion

Is it possible to process 2 secconds of audio at a time?

So that the visuals and audio sync up?

## Output

Stretch Objectives & Implementation Feasibility
1️⃣ Visual Scaling - Aspect Ratio Improvement
Current:

Y-axis height: 0.09
X-axis width: 130
Fix Proposal:

Increase x-axis scale further for a wider waveform.
Reduce y-axis range slightly to avoid clipping.
2️⃣ Audio-to-Video Playback Sync
Current:

Video finishes slightly ahead of audio.
Fix Proposal:

Reduce FPS by ~11/12ths
Formula: New_FPS = Current_FPS * (11/12)
Implementation: Adjust FPS = 440 instead of 480.
3️⃣ Frequency Analysis
✔ Feasible using Fast Fourier Transform (FFT).
✔ Extract dominant frequencies from FFT data.

Implementation Plan:

Find the peak frequency per frame.
Output the strongest frequency every frame.
4️⃣ Live Recording & Real-Time Playback
✔ Feasible using cpal crate for real-time audio input.
✔ Play processed video simultaneously with input sound.

Implementation Plan:

Capture microphone input.
Process it with FFT & visualization.
Ensure <50ms latency.
5️⃣ Instrument Chord Detection
✔ Feasible using Harmonic Product Spectrum (HPS).
✔ Detect multiple fundamental frequencies in real-time.

Implementation Plan:

Map detected frequencies to musical notes.
Compare notes to known chord structures.
Output estimated chord (e.g., C Major, D minor).
6️⃣ Package Code into Modules
✔ Feasible - Refactor into multiple files.

Implementation Plan:

audio.rs → Audio handling & FFT
visualization.rs → GUI & waveform plotting
main.rs → Entry point
7️⃣ Frequency Display (5 Hz - 20 kHz)
✔ Feasible with logarithmic frequency scaling.
✔ egui::plot::LogScale can display correct spectrum.

Implementation Plan:

X-axis → Logarithmic frequency scale
Y-axis → Amplitude representation
8️⃣ User Input - GUI Slider for Volume Control
✔ Feasible with egui::Slider.

Implementation Plan:

Add Volume (%) slider in GUI.
Modify playback volume dynamically.
9️⃣ Sync Processing Suggestion - 2s Batching
✔ Feasible by buffering audio in 2-second chunks.

Implementation Plan:

Process every 2s of data at once.
Plot 2s worth of waveform before moving forward.
Ensure FPS remains adaptive.
📌 Next Steps
Reduce FPS to 440 (~11/12th speed).
Add frequency detection for real-time analysis.
Implement log-scale frequency visualization (5Hz - 20kHz).
Refactor into modules (audio.rs, visual.rs).
Add GUI slider for real-time volume control.
Prototype 2-second batch processing for synchronization.
🚀 Stretch Goal
🎯 Live chord detection for instruments (Guitar/Piano).
🎯 Implement HPS for note identification.
🎯 Output detected chord in GUI.

Would you like step-by-step implementation, or batch updates? 🚀

## Input

Not sure what batch updates are, so I will go with a step by step, but only one step first.  Please give file partitioning first as suggested.

## Output

Step 1️⃣: File Partitioning
We'll refactor the code into three files:

audio.rs → Handles audio playback & FFT processing
visualization.rs → Handles GUI & waveform plotting
main.rs → Entry point
📌 Instructions
1️⃣ Create a new file: src/audio.rs
2️⃣ Create a new file: src/visualization.rs
3️⃣ Modify main.rs to use these modules.

📂 File 1: audio.rs (Handles audio & FFT)
rust
Copy
Edit
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use hound;
use rodio::{Decoder, OutputStream, source::Source};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use std::fs::File;
use std::io::BufReader;

const CHUNK_SIZE: usize = 256;
const DOWNSAMPLE_FACTOR: usize = 8;
const FPS: usize = 240;

pub struct AudioVisualizer {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub is_playing: Arc<Mutex<bool>>,
}

impl AudioVisualizer {
    pub fn new(audio_duration_secs: f64) -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE / 2]));
        let is_playing = Arc::new(Mutex::new(true));

        let waveform_clone = Arc::clone(&waveform);
        let fft_result_clone = Arc::clone(&fft_result);
        let is_playing_clone = Arc::clone(&is_playing);

        let total_frames = (audio_duration_secs * FPS as f64) as usize;
        let time_per_frame = audio_duration_secs / total_frames as f64;

        thread::spawn(move || {
            let filename = "./test.wav";
            let reader = hound::WavReader::open(filename).expect("Failed to open file");

            let samples: Vec<f64> = reader.into_samples::<i16>()
                .filter_map(Result::ok)
                .map(|s| s as f64)
                .collect();

            let mut current_window: Vec<f64> = vec![0.0; CHUNK_SIZE * 5];
            let start_time = Instant::now();

            for (i, chunk) in samples.chunks(CHUNK_SIZE).enumerate() {
                let downsampled_chunk: Vec<f64> = chunk.iter()
                    .step_by(DOWNSAMPLE_FACTOR)
                    .cloned()
                    .collect();

                let shift_amount = (downsampled_chunk.len() * 5).min(current_window.len());
                current_window.drain(..shift_amount);
                current_window.extend(downsampled_chunk.clone());

                {
                    let mut waveform_data = waveform_clone.lock().unwrap();
                    waveform_data.clear();
                    waveform_data.extend(&current_window);
                }

                {
                    let mut fft_data = fft_result_clone.lock().unwrap();
                    *fft_data = Self::compute_fft(&current_window);
                }

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
        buffer.iter().take(len / 2).map(|c| c.norm()).collect()
    }
}
📂 File 2: visualization.rs (Handles GUI & Plotting)
rust
Copy
Edit
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
📂 File 3: main.rs (Entry Point)
rust
Copy
Edit
mod audio;
mod visualization;

use audio::AudioVisualizer;
use eframe::NativeOptions;
use eframe::epaint::vec2;
use eframe::egui;

fn main() {
    let filename = "./test.wav";
    let (_stream, stream_handle) = rodio::OutputStream::try_default().expect("Failed to create output stream");

    let file = std::fs::File::open(filename).expect("Failed to open file");
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).expect("Failed to decode audio");

    let reader = hound::WavReader::open(filename).expect("Failed to open file for duration check");
    let num_samples = reader.len() as f64;
    let sample_rate = reader.spec().sample_rate as f64;
    let audio_duration_secs = num_samples / sample_rate;

    let _ = stream_handle.play_raw(source.convert_samples());

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(vec2(800.0, 500.0)), // Adjusted aspect ratio
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Real-Time Audio FFT Visualizer",
        options,
        Box::new(move |_cc| Box::new(AudioVisualizer::new(audio_duration_secs))),  
    ) {
        eprintln!("Error running eframe: {}", e);
    };
}
📌 Steps to Apply Changes
Create the files src/audio.rs and src/visualization.rs.
Move the corresponding code to these files.
Update main.rs to mod audio; mod visualization; at the top.
Run with cargo run to test functionality.
📌 Expected Outcome
✔ No behavior change
✔ Code is now modular & structured
✔ Easier to add features (like frequency analysis & real-time recording)

🔹 Step complete.
Would you like to test & confirm before moving to step 2 (Adding Frequency Analysis)? 🚀

