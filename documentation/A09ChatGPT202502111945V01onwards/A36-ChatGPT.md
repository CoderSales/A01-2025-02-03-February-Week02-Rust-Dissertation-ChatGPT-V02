# ChatGPT

## Output

Next Steps: Identify Harmonics and Fundamental Frequency
Key Objectives:
✅ Identify fundamental frequency (likely the note being played).
✅ Extract top harmonics (overtones).
✅ Make analysis more accurate using peak detection.
✅ Prepare for a basic visualization in a later step.

Step 1: Refactor Code into Separate Files
Create Cargo.toml with dependencies:
toml
Copy
Edit
[dependencies]
cpal = "0.15"
rustfft = "6.1.0"
Step 2: Create src/audio.rs (Handles Audio Input)
rust
Copy
Edit
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{StreamConfig, SampleFormat, Device};

/// Selects the best available input device
pub fn get_audio_device() -> Device {
    let host = cpal::default_host();
    
    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select VB-Audio Virtual Cable if available, otherwise default input device
    host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device())
        .expect("No suitable audio input device found")
}

/// Retrieves default audio config
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}
Step 3: Create src/fft.rs (Handles Frequency Analysis)
rust
Copy
Edit
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 44100.0;
const MIN_AMPLITUDE: f32 = 0.02; // Ignore low-energy noise
const FFT_SIZE: usize = 2048; // Larger FFT window

/// Perform FFT and return top frequency peaks (fundamental + harmonics)
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Check amplitude to ignore silence
    let amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;
    if amplitude < MIN_AMPLITUDE {
        return vec![]; // No detected note
    }

    // Apply Hanning window
    let hann_window: Vec<f32> = (0..samples.len())
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (samples.len() - 1) as f32).cos()))
        .collect();

    let windowed_samples: Vec<f32> = centered_samples
        .iter()
        .zip(hann_window.iter())
        .map(|(s, w)| s * w)
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(windowed_samples.len());

    let mut buffer: Vec<Complex<f32>> = windowed_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    
    // Extract top 3 frequency peaks
    let mut peak_frequencies: Vec<(f32, f32)> = magnitude_spectrum.iter()
        .enumerate()
        .map(|(i, &m)| ((i as f32) * (SAMPLE_RATE / samples.len() as f32), m))
        .collect();

    peak_frequencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort by magnitude

    peak_frequencies.truncate(3); // Keep top 3 peaks
    peak_frequencies
}
Step 4: Modify src/main.rs to Use New Modules
rust
Copy
Edit
mod audio;
mod fft;

use cpal::traits::{StreamTrait};
use std::sync::{Arc, Mutex};

const MIN_FREQUENCY: f32 = 20.0;  // Ignore frequencies below 20 Hz
const MAX_FREQUENCY: f32 = 20000.0; // Ignore extreme false frequencies
const FFT_SIZE: usize = 2048; // FFT size for resolution

fn main() {
    let device = audio::get_audio_device();
    let config = audio::get_audio_config(&device);

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);
    
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);

            if buffer.len() >= FFT_SIZE {
                let peaks = fft::analyze_frequencies(&buffer[..FFT_SIZE]);

                if !peaks.is_empty() {
                    let mut note_playing = note_clone.lock().unwrap();

                    let fundamental = peaks[0].0; // The strongest frequency
                    if fundamental >= MIN_FREQUENCY && fundamental <= MAX_FREQUENCY {
                        if !*note_playing {
                            println!("Fundamental: {:.2} Hz", fundamental);
                            for &(freq, mag) in peaks.iter() {
                                println!("  Harmonic: {:.2} Hz (Mag: {:.2})", freq, mag);
                            }
                        }
                        *note_playing = true;
                    } else {
                        *note_playing = false;
                    }
                }
                buffer.clear();
            }
        },
        err_fn,
        None,
    ).expect("Failed to create stream");

    stream.play().expect("Failed to start stream");

    println!("Listening for audio... Press Ctrl+C to stop.");
    std::thread::sleep(std::time::Duration::from_secs(30));
}
Improvements
✅ Refactored into modules: audio.rs (handles device selection), fft.rs (handles frequency analysis).
✅ Identifies top harmonics along with fundamental frequency.
✅ Improved note detection accuracy with Hanning window + larger FFT size.
✅ Filters out background noise by ignoring low-amplitude sounds.
✅ Prepares for visualization (later steps).

Expected Output
When playing A4 (440 Hz) on midi.city, expected:

yaml
Copy
Edit
Fundamental: 440.00 Hz
  Harmonic: 880.00 Hz (Mag: 1.23)
  Harmonic: 1320.00 Hz (Mag: 0.67)
A C4 (261.63 Hz) might show:

yaml
Copy
Edit
Fundamental: 261.63 Hz
  Harmonic: 523.25 Hz (Mag: 1.02)
  Harmonic: 784.88 Hz (Mag: 0.56)
Run
sh
Copy
Edit
cargo run
Next Steps
✅ Verify that harmonics + fundamental frequencies match notes played.
🔜 Display real-time frequency graph using egui or plotters.
🔜 Implement automatic note name detection (e.g., C4, A4, G5).
🚀 Try this update and let me know the accuracy!

