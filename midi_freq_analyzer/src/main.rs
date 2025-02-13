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
    let device = select_audio_device();
    let config = audio::get_audio_config(&device);

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));
    let last_note = Arc::new(Mutex::new("".to_string())); // Track last note

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);
    let last_note_clone = Arc::clone(&last_note);

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
                    let mut last_note = last_note_clone.lock().unwrap();

                    let fundamental = peaks[0].0;
                    let adjusted_fundamental = subtract_noise(fundamental, &noise_profile);
                    let note_name = frequency_to_note(adjusted_fundamental);

                    if adjusted_fundamental >= MIN_FREQUENCY && adjusted_fundamental <= MAX_FREQUENCY {
                        let note_name = frequency_to_note(adjusted_fundamental);
                        if !*note_playing {
                            
                            // ✅ **Only print if the note has changed**
                            if *last_note != note_name {
                                println!("Adjusted Fundamental: {:.2} Hz ({})", adjusted_fundamental, note_name);
                                *last_note = note_name.clone();
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

/// **Subtract noise profile from frequency reading with proper limit**
fn subtract_noise(frequency: f32, noise_profile: &Vec<f32>) -> f32 {
    if noise_profile.is_empty() {
        return frequency;
    }

    // Calculate rolling noise average
    let weight_factor = 0.8; // Give 80% weight to past noise, 20% to current
    let rolling_noise_avg: f32 = noise_profile.iter().rev().take(10) // Use last 10 readings
        .sum::<f32>() / 10.0; 

    let adjusted = (frequency - rolling_noise_avg * weight_factor).max(20.0); // Adaptive subtraction

    if adjusted < MIN_FREQUENCY {
        return 0.0; // Ignore too-low frequencies
    }
    adjusted
}

/// **Capture a reliable noise profile by taking multiple readings**
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
        let mut raw_noise = fft::analyze_frequencies(&buffer[..2048])
            .iter()
            .map(|&(freq, _)| freq)
            .collect::<Vec<f32>>();

        if raw_noise.len() > 5 {
            raw_noise.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort for median calculation
            noise_samples = raw_noise[raw_noise.len() / 2..].to_vec(); // Keep only the higher half
        }
    }

    stream.pause().expect("Failed to pause stream");
    println!("Noise profile captured.");
    noise_samples
}

/// **Save noise profile to file**
fn save_noise_profile(noise_profile: &Vec<f32>) {
    if noise_profile.is_empty() {
        return;
    }

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

/// Converts a frequency to the closest musical note
fn frequency_to_note(frequency: f32) -> String {
    let a4_freq = 440.0;
    let semitone_ratio = 2.0_f32.powf(1.0 / 12.0);

    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
    ];

    let mut closest_note = "Unknown".to_string();
    let mut min_diff = f32::MAX;
    let mut best_index = 0;
    let mut best_octave = 4;

    for i in -48..=48 { // Covers ~4 octaves up/down
        let note_freq = a4_freq * semitone_ratio.powf(i as f32);
        let diff = (frequency - note_freq).abs();

        if diff < min_diff {
            min_diff = diff;
            best_index = ((i + 9) % 12) as usize;
            best_octave = 4 + (i + 9) / 12;
        }
    }

    // Ensure the index is within bounds
    if best_index < note_names.len() {
        closest_note = format!("{}{}", note_names[best_index], best_octave);
    }

    closest_note
}

