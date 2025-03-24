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
mod config;

mod helpers;
use helpers::spawn_audio_thread;
use helpers::spawn_logger_thread;
use helpers::create_panicked_threads;
use helpers::select_input_device;
use helpers::launch_gui_safely;



fn main() {
    print_input_devices(); // always runs at start

    let panicked_threads = create_panicked_threads();
    let panicked_threads_clone = Arc::clone(&panicked_threads);

    let output_gain = Arc::new(Mutex::new(1.0));
    let input_gain = Arc::new(Mutex::new(1.0));

    // 👇 Spawn background audio thread using cloned gains

    spawn_audio_thread(&panicked_threads_clone, &output_gain, &input_gain);

    // 👇 GUI uses same gains
    launch_gui_safely(output_gain, input_gain);


    let program_start = Instant::now(); // ✅ Fix: Declare inside main()
    let host = cpal::default_host(); // ✅ Define `host` first
            
    // ✅ Move logging into a separate thread
    
    let device = select_input_device();
    spawn_logger_thread(program_start);
}

