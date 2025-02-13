# ChatGPT

## Input

### Issue

Compiles and runs but

no frequencies detected when
key is played.

### Previous working code

use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{StreamTrait, DeviceTrait}; // ✅ Import DeviceTrait
use std::sync::{Arc, Mutex};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const FFT_SIZE: usize = 2048;

fn main() {
    let device = audio::get_audio_device();
    let config = audio::get_audio_config(&device);

    println!("\nUsing input device: {}\n", device.name().unwrap()); // ✅ Now works

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);

    let stream = device.build_input_stream( // ✅ Now works
        &config,
        move |data: &[f32], _: &_| {
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);

            if buffer.len() >= FFT_SIZE {
                let peaks = fft::analyze_frequencies(&buffer[..FFT_SIZE]);

                if !peaks.is_empty() {
                    let mut note_playing = note_clone.lock().unwrap();

                    let fundamental = peaks[0].0;
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

### New code with noise detection not working

use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{StreamTrait, DeviceTrait};
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const NOISE_PROFILE_FILE: &str = "noise_profile.txt";

fn main() {
    let device = audio::get_audio_device();
    let config = audio::get_audio_config(&device);

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);

    // Step 1: Capture Baseline Noise
    let noise_profile = if let Ok(profile) = load_noise_profile() {
        println!("Loaded saved noise profile.");
        profile
    } else {
        println!("Capturing noise profile...");
        let profile = capture_noise_profile(&device, &config);
        save_noise_profile(&profile);
        profile
    };

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);

            if buffer.len() >= 2048 {
                let peaks = fft::analyze_frequencies(&buffer[..2048]);

                if !peaks.is_empty() {
                    let mut note_playing = note_clone.lock().unwrap();
                    
                    let fundamental = peaks[0].0;
                    let adjusted_fundamental = subtract_noise(fundamental, &noise_profile);

                    if adjusted_fundamental >= MIN_FREQUENCY && adjusted_fundamental <= MAX_FREQUENCY {
                        if !*note_playing {
                            println!("Adjusted Fundamental: {:.2} Hz", adjusted_fundamental);
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

/// **Subtract noise profile from frequency reading**
fn subtract_noise(frequency: f32, noise_profile: &Vec<f32>) -> f32 {
    let noise_avg = noise_profile.iter().sum::<f32>() / noise_profile.len() as f32;
    (frequency - noise_avg).max(0.0) // Ensure non-negative frequency
}

/// **Capture and compute the baseline noise profile**
fn capture_noise_profile(device: &cpal::Device, config: &cpal::StreamConfig) -> Vec<f32> {
    let mut noise_samples = Vec::new();
    let data = Arc::new(Mutex::new(Vec::new()));

    let data_clone = Arc::clone(&data);
    let err_fn = |err| eprintln!("Error: {:?}", err);

    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &_| {
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);
        },
        err_fn,
        None,
    ).expect("Failed to create stream");

    stream.play().expect("Failed to start stream");

    println!("Capturing noise for 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let buffer = data.lock().unwrap();
    if buffer.len() >= 2048 {
        noise_samples = fft::analyze_frequencies(&buffer[..2048])
            .iter()
            .map(|&(freq, _)| freq)
            .collect();
    }

    stream.pause().expect("Failed to pause stream");
    noise_samples
}

/// **Save noise profile to file**
fn save_noise_profile(noise_profile: &Vec<f32>) {
    let mut file = File::create(NOISE_PROFILE_FILE).expect("Failed to create noise profile file");
    for freq in noise_profile {
        writeln!(file, "{}", freq).expect("Failed to write to noise profile file");
    }
    println!("Noise profile saved.");
}

/// **Load noise profile from file**
fn load_noise_profile() -> Result<Vec<f32>, std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(NOISE_PROFILE_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let noise_profile: Vec<f32> = content.lines()
        .filter_map(|line| line.parse::<f32>().ok())
        .collect();
    
    Ok(noise_profile)
}

### Revert to old code

use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{StreamTrait, DeviceTrait}; // ✅ Import DeviceTrait
use std::sync::{Arc, Mutex};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const FFT_SIZE: usize = 2048;

fn main() {
    let device = audio::get_audio_device();
    let config = audio::get_audio_config(&device);

    println!("\nUsing input device: {}\n", device.name().unwrap()); // ✅ Now works

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);

    let stream = device.build_input_stream( // ✅ Now works
        &config,
        move |data: &[f32], _: &_| {
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);

            if buffer.len() >= FFT_SIZE {
                let peaks = fft::analyze_frequencies(&buffer[..FFT_SIZE]);

                if !peaks.is_empty() {
                    let mut note_playing = note_clone.lock().unwrap();

                    let fundamental = peaks[0].0;
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
