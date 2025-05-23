use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
#[allow(unused)]
use std::sync::atomic::AtomicBool;
#[allow(unused)]
use std::sync::atomic::Ordering;
use cpal::{Device, StreamConfig};
use cpal::traits::DeviceTrait;
use cpal::traits::StreamTrait;
// use crate::fft;
use crate::live_output;
// use crate::BUFFER_SIZE;


// use crate::audio_playback::play_audio;  // ✅ Use crate:: to refer to the correct module
// use audio_playback::play_audio;  // ✅ Import function

use super::audio_playback::play_audio;


const NOISE_PROFILE_FILE: &str = "noise_profile.txt";



/// **Load noise profile from file**
pub fn load_noise_profile() -> Result<Vec<f32>, std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(NOISE_PROFILE_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let noise_profile: Vec<f32> = content.lines()
        .filter_map(|line| line.parse::<f32>().ok())
        .collect();

    Ok(noise_profile)
}

/// **Capture a reliable noise profile by taking multiple readings**
pub fn capture_noise_profile(device: &cpal::Device, config: &cpal::StreamConfig) -> Vec<f32> {
    let noise_samples = Vec::new();
    let data = Arc::new(Mutex::new(Vec::new()));

    let data_clone = Arc::clone(&data);
    #[allow(unused)]
    let err_fn: Box<dyn Fn(cpal::StreamError) + Send> = Box::new(|err| eprintln!("Error: {:?}", err));

    let stream = device.build_input_stream(
        &config,  
        {
            let data_clone = Arc::clone(&data_clone);
            move |data: &[f32], _: &_| {
                for &sample in data {
                    let amplitude = sample.abs();
                    live_output::print_live_amplitude(amplitude);
                }

                if let Ok(mut buffer) = data_clone.lock() {
                    buffer.extend_from_slice(data);
                } else {
                    static mut ERROR_COUNT: usize = 0;
                    unsafe {
                        ERROR_COUNT += 1;
                        if ERROR_COUNT % 10 == 0 {
                            eprintln!("⚠️ Skipping buffer update due to PoisonError ({} occurrences)", ERROR_COUNT);
                        }
                    }
                }
            }
        },
        move |err| eprintln!("Stream error: {:?}", err),
        None,
    ).expect("Failed to create stream");

    stream.play().expect("Failed to start stream");

    println!("🔊 Running 30ms Audio Processing Cycle... Press Ctrl+C to stop.");

    // ✅ Call the new playback function
    play_audio(Arc::clone(&data));

    stream.pause().expect("Failed to pause stream");
    println!("Noise profile captured.");
    noise_samples
}

/// **Get or capture noise profile**
pub fn get_or_capture_noise_profile(device: &Device, config: &StreamConfig) -> Vec<f32> {
    match load_noise_profile() {
        Ok(profile) => {
            println!("Loaded saved noise profile.");
            profile
        }
        Err(_) => {
            println!("Capturing noise profile...");
            let profile = capture_noise_profile(device, config);
            save_noise_profile(&profile);
            profile
        }
    }
}

/// **Save noise profile to file**
pub fn save_noise_profile(noise_profile: &Vec<f32>) {
    if noise_profile.is_empty() {
        return;
    }

    let mut file = File::create(NOISE_PROFILE_FILE).expect("Failed to create noise profile file");
    for freq in noise_profile {
        writeln!(file, "{}", freq).expect("Failed to write to noise profile file");
    }
    println!("Noise profile saved.");
}

