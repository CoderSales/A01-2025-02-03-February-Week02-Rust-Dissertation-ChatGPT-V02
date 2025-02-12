use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::{Arc, Mutex};

const SAMPLE_RATE: f32 = 44100.0; // Standard audio sample rate
const MIN_FREQUENCY: f32 = 20.0;  // Ignore frequencies below 20 Hz
const MAX_FREQUENCY: f32 = 20000.0; // Ignore extreme false frequencies
const MIN_AMPLITUDE: f32 = 0.01; // Ignore silence / very low volumes

fn main() {
    let host = cpal::default_host();

    // Print available input/output devices
    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }
    
    println!("\nAvailable output devices:");
    for device in host.output_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select input device (Virtual Cable preferred)
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

                if buffer.len() >= 1024 {
                    let freq = analyze_frequency(&buffer[..1024]);

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

/// Analyze frequency using FFT
fn analyze_frequency(samples: &[f32]) -> f32 {
    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Check amplitude threshold to detect silence
    let amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;
    if amplitude < MIN_AMPLITUDE {
        return 0.0; // Return 0 Hz when no note is played
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(centered_samples.len());

    let mut buffer: Vec<Complex<f32>> = centered_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    let max_index = magnitude_spectrum.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

    let frequency = (max_index as f32) * (SAMPLE_RATE / samples.len() as f32);
    frequency
}
