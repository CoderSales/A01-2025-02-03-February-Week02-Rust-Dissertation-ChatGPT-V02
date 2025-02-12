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
