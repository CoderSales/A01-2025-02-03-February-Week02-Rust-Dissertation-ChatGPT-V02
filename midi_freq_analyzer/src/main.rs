mod constants;
mod audio_io;
mod stream_setup;
mod noise;
mod notes;
mod analysis;

use constants::BUFFER_SIZE;
use audio_io::start_audio_io;
use stream_setup::setup_audio_stream;
use noise::subtract_noise;
use notes::frequency_to_note;
use analysis::analyze_amplitude;


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
// const BUFFER_SIZE: usize = 2048;


// let output_size = output_config.buffer_size().unwrap_or(960); // fallback
// let buffer = create_buffer(output_size);

// let _host = cpal::default_host();
// let input_device = cpal::default_host().default_input_device().expect("No default input device");
// let output_device = cpal::default_host().default_output_device().expect("No default output device");
// // println!("\n🎤 Selected Input Device: {}", input_device.name().unwrap());
// // println!("🔊 Selected Output Device: {}", output_device.name().unwrap());
// let output_config = audio::get_audio_config(&output_device);// ✅ Define config first
// bitrate::print_audio_bitrate(&output_config);



// const BUFFER_SIZE: usize = output_size; // or just remove this const entirely // ❌ INVALID
// const BUFFER_SIZE: usize = 960;
mod buffer_handling;
use buffer_handling::handle_buffer_lock;
mod thread_manager;
use thread_manager::spawn_thread;
mod mutex_handling;
use mutex_handling::*;
mod device_selection;
use crate::noise_profile::get_or_capture_noise_profile;
use crate::fft::analyze_frequencies;

mod list_inputs; // add at top
use crate::list_inputs::print_input_devices;




fn main() {
    print_input_devices(); // always runs at start

    start_audio_io(); // call directly, not in thread
    gui::launch_gui().unwrap(); // optional: run after

    let panicked_threads = create_panicked_threads();
    let panicked_threads_clone = Arc::clone(&panicked_threads);
    spawn_thread(move || {
        let thread_name = "Audio Processing Thread".to_string();
        if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            audio_io::start_audio_io();
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
}

