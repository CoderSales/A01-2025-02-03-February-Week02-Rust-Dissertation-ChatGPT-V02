use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
static mut PRINT_COUNTER: usize = 0; 
use std::time::{Instant, Duration};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use lua_ui::init_lua_ui;
mod live_output;
mod bitrate;
mod gui;
mod lua_ui;
mod noise_profile;
const BUFFER_SIZE: usize = 2048;
mod buffer_handling;
use buffer_handling::handle_buffer_lock;
mod thread_manager;
use thread_manager::spawn_thread;
mod mutex_handling;
use mutex_handling::*;
mod device_selection;
use crate::noise_profile::get_or_capture_noise_profile;
use crate::fft::analyze_frequencies;

fn start_audio_io() {
    let _host = cpal::default_host();
    let input_device = cpal::default_host().default_input_device().expect("No default input device");
    let output_device = cpal::default_host().default_output_device().expect("No default output device");
    // println!("\n🎤 Selected Input Device: {}", input_device.name().unwrap());
    // println!("🔊 Selected Output Device: {}", output_device.name().unwrap());
    let config = audio::get_audio_config(&output_device);// ✅ Define config first
    bitrate::print_audio_bitrate(&config);
    println!("\nUsing input device: {}\n", input_device.name().unwrap());
    // let data = create_shared_data();
    // let note_playing = create_note_playing();
    // let last_note = create_last_note(); // Track last note
    // let noise_profile = get_or_capture_noise_profile(&input_device, &config);
    let buffer = create_buffer(BUFFER_SIZE); // ✅ Ensure buffer exists
    let buffer_clone = Arc::clone(&buffer); // ✅ Clone before using
    let sample_rate = config.sample_rate.0; // ✅ Fix: Remove `()` & move before stream creation
    let stream = output_device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                let buffer = buffer_clone.lock().unwrap();
                let safe_len = buffer.len().min(data.len()); // ✅ Prevent out-of-bounds errors
                data.copy_from_slice(&buffer[..safe_len]); // ✅ Safe indexing
            },
            move |err| eprintln!("Stream error: {:?}", err),
            None,
        )
        .unwrap();
    stream.play().unwrap();
    println!("Using output device: {}", output_device.name().unwrap());
    spawn_thread(move || {
        let buffer_clone = Arc::clone(&buffer);
        loop {
            handle_buffer_lock(&buffer_clone, |buffer| {
                for i in 0..BUFFER_SIZE {
                    buffer[i] = (i as f32 / sample_rate as f32).sin();
                }
            });
                        thread::sleep(Duration::from_millis(10));
        }
    });
    loop {
        thread::sleep(Duration::from_secs(1)); // Keep main thread alive
    }
}


fn main() {
    let panicked_threads = create_panicked_threads();
    let panicked_threads_clone = Arc::clone(&panicked_threads);
    spawn_thread(move || {
        let thread_name = "Audio Processing Thread".to_string();
        if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            start_audio_io();
        })) {
            eprintln!("⚠️ Thread panicked: {}", thread_name);
            let mut list = panicked_threads_clone.lock().unwrap();
            list.insert(thread_name);
        }
    }); // Run audio processing in background

    // launch_gui(); // Run GUI (Audio Analyzer + Frequency Meter)

    if let Err(e) = gui::launch_gui() {
        eprintln!("GUI failed: {:?}", e);
    }
    
    // Define options and app before calling eframe::run_native():
    let options = eframe::NativeOptions::default(); 
    let app = gui::AudioApp::default();  
    
    eframe::run_native(
        "Audio Analyzer",
        options.clone(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
    

    let program_start = Instant::now(); // ✅ Fix: Declare inside main()
    let host = cpal::default_host(); // ✅ Define `host` first
            
    // ✅ Move logging into a separate thread
    
    let device = device_selection::select_audio_device(true);
    spawn_thread(move || {
        loop {
            let elapsed = program_start.elapsed().as_secs();
            if elapsed % 5 == 0 {
                println!("⏳ Program Running: {} seconds elapsed.", elapsed);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let config = audio::get_audio_config(&device); // ✅ Define config first

    bitrate::print_audio_bitrate(&config);

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let data = create_shared_data();
    let note_playing = create_note_playing();
    let last_note = create_last_note(); // Track last note

    // let err_fn: Box<dyn Fn(cpal::StreamError)> = Box::new(|err| eprintln!("Error: {:?}", err));

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);
    let last_note_clone = Arc::clone(&last_note);

    let noise_profile = get_or_capture_noise_profile(&device, &config);

    // Edited: Ensure display_amplitude() is called live inside input stream processing
    let stream = setup_audio_stream(&device, &config, Arc::clone(&data));
    stream.play().unwrap();
}


fn setup_audio_stream(device: &cpal::Device, config: &cpal::StreamConfig, data_clone: Arc<Mutex<Vec<f32>>>) -> cpal::Stream {
    device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let (_low, _mid, _high) = analyze_frequencies(data);
            
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);

            if buffer.len() > BUFFER_SIZE {
                let len = buffer.len();
                buffer.drain(..len - BUFFER_SIZE);
}
        },
        move |err| eprintln!("Stream error: {:?}", err),
        None,
    ).expect("Failed to create stream")
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

fn analyze_amplitude(samples: &[f32]) {
    static mut LAST_ANALYSIS_TIME: Option<Instant> = None;

    let now = Instant::now();
    unsafe {
        if let Some(last_time) = LAST_ANALYSIS_TIME {
            if now.duration_since(last_time) < Duration::from_secs(5) {
                return;  // Skip print if less than 5 seconds since last output
            }
        }
        LAST_ANALYSIS_TIME = Some(now);
    }

    let min = samples.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = samples.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;

    let mut sorted_samples = samples.to_vec();
    sorted_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted_samples.len() % 2 == 0 {
        (sorted_samples[sorted_samples.len() / 2 - 1] + sorted_samples[sorted_samples.len() / 2]) / 2.0
    } else {
        sorted_samples[sorted_samples.len() / 2]
    };

    println!(
        "🔍 Amplitude Analysis - Min: {:.5}, Max: {:.5}, Mean: {:.5}, Median: {:.5}",
        min, max, mean, median
    );

}
