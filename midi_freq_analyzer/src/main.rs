#[allow(unused)]
mod constants;
mod audio_io;
mod stream_setup;
mod noise;
mod notes;
mod analysis;
#[allow(unused)]
use crate::config::BUFFER_SIZE; // use constants::BUFFER_SIZE;
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
use constants::{MIN_FREQUENCY, MAX_FREQUENCY};
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
use crate::fft::fft_analyze_frequencies::analyze_frequencies;

mod list_inputs; // add at top
use crate::list_inputs::print_input_devices;

use midi_freq_analyzer::gui_main::{launch_gui, AudioApp};
use crate::visualization::Visualization;
mod config;

mod helpers;
use helpers::spawn_audio_thread;
use helpers::spawn_logger_thread;
use helpers::create_panicked_threads;
use helpers::select_input_device;
use helpers::launch_gui_safely;
use helpers::create_gain_controls;

use midi_freq_analyzer::output_handler::print_cli_line;
use midi_freq_analyzer::audio_io::start_audio_io;
use midi_freq_analyzer::output_handler::*;

mod output_handler;
mod visualization;
mod audio2;



// fn main() {
//     println!("🎬 Creating Visualization app...");

//     // 👇 Run CLI audio logic in background thread
//     std::thread::spawn(|| {
//         print_cli_line("hello test");
//         print_input_devices();

//         let panicked_threads = create_panicked_threads();
//         let (output_gain, input_gain) = create_gain_controls();
//         let clone = Arc::clone(&panicked_threads);
//         spawn_audio_thread(&clone, &output_gain, &input_gain);
//     });

//     // 👇 Run GUI
//     let app = Visualization::new();
//     let native_options = eframe::NativeOptions::default();

//     if let Err(e) = eframe::run_native(
//         "Audio Analyzer",
//         native_options,
//         Box::new(|_cc| Ok(Box::new(app))),
//     ) {
//         eprintln!("Error running eframe: {}", e);
//     }


//     // print_cli_line("hello test");
//     print_input_devices(); // always runs at start

//     let panicked_threads = create_panicked_threads();
//     let panicked_threads_clone = Arc::clone(&panicked_threads);

//     let (output_gain, input_gain) = create_gain_controls();

//     // 👇 Spawn background audio thread using cloned gains

//     // spawn_audio_thread(&panicked_threads_clone, &output_gain, &input_gain);

//     // 👇 GUI uses same gains
//     // launch_gui_safely(output_gain, input_gain);
    
    
//     // use visualization::Visualization;
    


//     // let native_options = eframe::NativeOptions::default();
    
    
    
//     // eframe::run_native("Audio Analyzer", native_options, Box::new(|_| Ok(Box::new(Visualization::new()))));
    

//     let program_start = Instant::now(); // ✅ Fix: Declare inside main()
//     // let host = cpal::default_host(); // ✅ Define `host` first
            
//     // // ✅ Move logging into a separate thread
    
//     // let device = select_input_device();
//     spawn_logger_thread(program_start);
// }

// ------------------------------------------------------------------

// fn main() {
//     println!("🎬 Creating Visualization app...");

//     let program_start = Instant::now(); // 👈 place here

//     // CLI background
//     std::thread::spawn(move || {
//         print_cli_line("hello test");
//         print_input_devices();

//         let panicked_threads = create_panicked_threads();
//         let (output_gain, input_gain) = create_gain_controls();
//         let clone = Arc::clone(&panicked_threads);
//         spawn_audio_thread(&clone, &output_gain, &input_gain);

//         spawn_logger_thread(program_start);
//     });

//     // GUI
//     let app = Visualization::new();
//     let native_options = eframe::NativeOptions::default();
//     if let Err(e) = eframe::run_native(
//         "Audio Analyzer",
//         native_options,
//         Box::new(|_cc| Ok(Box::new(app))),
//     ) {
//         eprintln!("Error running eframe: {}", e);
//     }
// }

fn main() {
    let app = Visualization::new();
    let native_options = eframe::NativeOptions::default();
    
    if let Err(e) = eframe::run_native(
        "Audio Analyzer",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    ) {
        eprintln!("Error running eframe: {}", e);
    }    
}

// struct BasicApp;

// impl eframe::App for BasicApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.label("🚀 GUI is working");
//         });
//     }
// }
