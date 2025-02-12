# ChatGPT

## Input

Dominant frequency on first run is

0.00 Hz

on second run is

44056.93 Hz, which seems like possibly an issue with

sampling rate.

Please edit to only print if a note is detected abbove a certain threshold.

## Output

Fix: Ignore Low Frequencies & Remove DC Offset
Adjust the script to:

Ignore low frequencies (set a minimum threshold to filter out noise).
Remove DC offset by centering the signal before FFT.
1. Update src/main.rs:
rust
Copy
Edit
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::{Arc, Mutex};

const SAMPLE_RATE: f32 = 44100.0; // Standard audio sample rate
const MIN_FREQUENCY: f32 = 20.0;  // Ignore frequencies below 20 Hz (human hearing range)

fn main() {
    let host = cpal::default_host();
    let device = host
        .input_devices()
        .expect("Failed to get input devices")
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .expect("VB-Audio Virtual Cable not found");

    println!("Using input device: {}", device.name().unwrap());

    let config = device.default_input_config().unwrap();
    let sample_format = config.sample_format();
    let stream_config: StreamConfig = config.into();

    let data = Arc::new(Mutex::new(Vec::new()));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &_| {
                let mut buffer = data_clone.lock().unwrap();
                buffer.extend_from_slice(data);

                if buffer.len() >= 1024 {
                    let freq = analyze_frequency(&buffer[..1024]);
                    if freq >= MIN_FREQUENCY {
                        println!("Detected frequency: {:.2} Hz", freq);
                    }
                    buffer.clear();
                }
            },
            err_fn,
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .expect("Failed to create stream");

    stream.play().expect("Failed to start stream");

    println!("Listening for audio... Press Ctrl+C to stop.");
    std::thread::sleep(std::time::Duration::from_secs(10));
}

/// Analyze frequency using FFT
fn analyze_frequency(samples: &[f32]) -> f32 {
    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(centered_samples.len());

    let mut buffer: Vec<Complex<f32>> = centered_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    let max_index = magnitude_spectrum.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

    let frequency = (max_index as f32) * (SAMPLE_RATE / samples.len() as f32);
    frequency
}
Fixes:
DC Offset Removal: Centers the signal before FFT by subtracting the mean.
Minimum Frequency Threshold: Ignores frequencies below 20 Hz to remove noise.
Expected Output:
Only real notes should be printed, e.g.:

yaml
Copy
Edit
Detected frequency: 440.00 Hz
Detected frequency: 261.63 Hz
Run:

sh
Copy
Edit
cargo run
Next step: Verify frequency accuracy on midi.city 🎵

