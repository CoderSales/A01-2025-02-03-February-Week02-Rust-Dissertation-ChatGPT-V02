#[allow(unused)]
mod constants;
mod audio_io;
mod stream_setup;
mod noise;
mod notes;
mod analysis;
#[allow(unused)]
use constants::BUFFER_SIZE;
#[allow(unused)]
use audio_io::start_audio_io;
#[allow(unused)]
use stream_setup::setup_audio_stream;
#[allow(unused)]
use noise::subtract_noise;
#[allow(unused)]
use notes::frequency_to_note;
#[allow(unused)]
use analysis::analyze_amplitude;

#[allow(unused)]
use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
#[allow(unused)]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
#[allow(unused)]
use std::thread;
#[allow(unused)]
use std::fs::{File, OpenOptions};
#[allow(unused)]
use std::io::{Read, Write};
const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
static mut PRINT_COUNTER: usize = 0; 
#[allow(unused)]
use std::time::{Instant, Duration};
#[allow(unused)]
use std::sync::atomic::AtomicBool;
#[allow(unused)]
use std::sync::atomic::Ordering;
#[allow(unused)]
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
#[allow(unused)]
mod buffer_handling;
use buffer_handling::handle_buffer_lock;
mod thread_manager;
use thread_manager::spawn_thread;
mod mutex_handling;
use mutex_handling::*;
mod device_selection;
#[allow(unused)]
use crate::noise_profile::get_or_capture_noise_profile;
#[allow(unused)]
use crate::fft::analyze_frequencies;

mod list_inputs; // add at top
use crate::list_inputs::print_input_devices;

use midi_freq_analyzer::gui_main::{launch_gui, AudioApp};



fn main() {
    print_input_devices(); // always runs at start

    let panicked_threads = create_panicked_threads();
    let panicked_threads_clone = Arc::clone(&panicked_threads);

    let output_gain = Arc::new(Mutex::new(1.0));
    let input_gain = Arc::new(Mutex::new(1.0));

    // 👇 Spawn background audio thread using cloned gains
    spawn_thread({
        let output_gain = Arc::clone(&output_gain);
        let input_gain = Arc::clone(&input_gain);
        move || {
            let thread_name = "Audio Processing Thread".to_string();
            if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                audio_io::start_audio_io(output_gain, input_gain);
            })) {
                eprintln!("⚠️ Thread panicked: {}", thread_name);
                let mut list = panicked_threads_clone.lock().unwrap();
                list.insert(thread_name);
            }
        }
    });

    // 👇 GUI uses same gains
    if let Err(e) = launch_gui(output_gain, input_gain) {
        eprintln!("GUI failed: {:?}", e);
    }
    
    // Define options and app before calling eframe::run_native():
    // let options = eframe::NativeOptions::default(); 
    // let app = AudioApp::default();  
    
    // eframe::run_native(
    //     "Audio Analyzer",
    //     options.clone(),
    //     Box::new(|_cc| Ok(Box::new(app))),
    // )
    // .unwrap();
    

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

