# ChatGPT

## Output

Next Step: Improve Frequency Accuracy
Issues Identified
Incorrect frequency detection for different notes (clustering around 86 Hz).
Possible aliasing or harmonic misidentification in FFT.
Fixes
✅ Use a larger FFT window (2048 samples instead of 1024)
✅ Use Hanning window to reduce spectral leakage
✅ Filter out low-energy frequencies to ignore background noise

Updated Code (Improved Accuracy)
Modify src/main.rs:

rust
Copy
Edit
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 44100.0; // Standard sample rate
const MIN_FREQUENCY: f32 = 20.0;  // Ignore frequencies below 20 Hz
const MAX_FREQUENCY: f32 = 20000.0; // Ignore extreme false frequencies
const MIN_AMPLITUDE: f32 = 0.02; // Ignore low-volume noise
const FFT_SIZE: usize = 2048; // Larger FFT window for better resolution

fn main() {
    let host = cpal::default_host();

    // Print available devices
    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    println!("\nAvailable output devices:");
    for device in host.output_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select preferred input device (VB-Audio Virtual Cable)
    let device = host
        .input_devices()
        .expect("Failed to get input devices")
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device())
        .expect("No suitable audio input device found");

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let config = device.default_input_config().unwrap();
    let sample_format = config.sample_format();
    let stream_config: StreamConfig = config.into();

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);
    
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &_| {
                let mut buffer = data_clone.lock().unwrap();
                buffer.extend_from_slice(data);

                if buffer.len() >= FFT_SIZE {
                    let freq = analyze_frequency(&buffer[..FFT_SIZE]);

                    let mut note_playing = note_clone.lock().unwrap();
                    
                    if freq >= MIN_FREQUENCY && freq <= MAX_FREQUENCY {
                        if !*note_playing {
                            println!("Detected frequency: {:.2} Hz", freq);
                        }
                        *note_playing = true;
                    } else {
                        *note_playing = false;
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
    std::thread::sleep(std::time::Duration::from_secs(30));
}

/// Apply Hanning window and compute FFT to detect dominant frequency
fn analyze_frequency(samples: &[f32]) -> f32 {
    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Check amplitude to ignore silence
    let amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;
    if amplitude < MIN_AMPLITUDE {
        return 0.0; // Ignore background noise
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
    let max_index = magnitude_spectrum.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

    let frequency = (max_index as f32) * (SAMPLE_RATE / samples.len() as f32);
    frequency
}
Fixes & Improvements
✅ Increased FFT size (2048 samples) → Better resolution, reducing aliasing errors.
✅ Applied Hanning window → Reduces frequency leakage (more accurate notes).
✅ Filtered out background noise → Ignores low-energy signals below 0.02 amplitude.

Expected Output
When playing a C4 (261.63 Hz) or A4 (440 Hz) on midi.city, output should be:

yaml
Copy
Edit
Detected frequency: 261.63 Hz
Detected frequency: 440.00 Hz
Each note should match actual frequency values rather than repeating around 86 Hz.

Run
sh
Copy
Edit
cargo run
Next Steps
✅ Confirm if notes now display correct frequencies
🔜 Implement real-time frequency visualization
🔜 Allow user to select input device dynamically
🚀 Try it out and let me know the results!
