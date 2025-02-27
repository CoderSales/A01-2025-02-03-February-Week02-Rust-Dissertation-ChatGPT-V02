# Chat

## Out

Issues Noted
1️⃣ Duplicate Import of Duration
Error: Duration is imported twice in main.rs.
Fix: Remove the second import (use std::time::{Instant, Duration};) or keep only use std::time::Instant;.
2️⃣ launch_gui Not Found
Error: Function launch_gui is called in main.rs, but it is not in scope.
Fix: Ensure launch_gui is correctly imported from gui.rs by adding:
rust
Copy
Edit
use crate::gui::launch_gui;
Or modify the function call to gui::launch_gui(); if mod gui; is already in place.
3️⃣ GUI thread::spawn Issues (Send Trait)
Error: eframe::run_native() inside thread::spawn() cannot be sent between threads.
Cause: NativeOptions contains non-Send components, so it cannot be passed into a new thread.
Fix: Move eframe::run_native() out of thread::spawn() and run it directly in main:
rust
Copy
Edit
eframe::run_native(
    "Audio Analyzer",
    options.clone(),
    Box::new(|_cc| Ok(Box::new(app))),
).unwrap();
4️⃣ build_output_stream() Missing Argument
Error: cpal::build_output_stream() now requires 4 arguments, but only 3 are provided.
Fix: Add the missing timeout parameter (None for default behavior):
rust
Copy
Edit
.build_output_stream(
    &config.into(),
    move |data: &mut [f32], _| {
        let mut buffer = buffer.lock().unwrap();
        data.copy_from_slice(&buffer[..data.len()]);
    },
    move |err| eprintln!("Stream error: {:?}", err),
    None, // ✅ Fix: Add this argument
)
Summary of Fixes
Issue	Fix
Duplicate Duration import	Remove one import.
launch_gui not found	Add use crate::gui::launch_gui;.
GUI thread safety (Send trait error)	Run eframe::run_native() without thread::spawn.
build_output_stream missing argument	Add None as the 4th argument.
Next Steps
Apply Fixes.
Run:
bash
Copy
Edit
cargo check
Confirm Errors Are Resolved.
Then Try:
bash
Copy
Edit
cargo run

## In

### CL Output

```bash
$ cargo check
    Checking midi_freq_analyzer v0.1.0 (C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer)
error[E0425]: cannot find value `options` in this scope
  --> src/main.rs:79:9
   |
79 |         options.clone(),
   |         ^^^^^^^ not found in this scope

error[E0425]: cannot find value `app` in this scope
  --> src/main.rs:80:36
   |
80 |         Box::new(|_cc| Ok(Box::new(app))),
   |                                    ^^^ not found in this scope

warning: unused variable: `app`
  --> src\gui.rs:86:9
   |
86 |     let app = AudioApp {
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_app`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:38:21
   |
38 |                 let mut buffer = buffer.lock().unwrap();
   |                     ----^^^^^^
   |                     |
   |                     help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

error[E0382]: use of moved value: `buffer`
  --> src/main.rs:48:19
   |
32 |     let buffer = Arc::new(Mutex::new(vec![0.0f32; buffer_size]));
   |         ------ move occurs because `buffer` has type `Arc<std::sync::Mutex<std::vec::Vec<f32>>>`, which does not implement the `Copy` trait
...
37 |             move |data: &mut [f32], _| {
   |             -------------------------- value moved into closure here
38 |                 let mut buffer = buffer.lock().unwrap();
   |                                  ------ variable moved due to use in closure
...
48 |     thread::spawn(move || {
   |                   ^^^^^^^ value used here after move
...
51 |                 let mut buffer = buffer.lock().unwrap();
   |                                  ------ use occurs due to use in closure

Some errors have detailed explanations: E0382, E0425.
For more information about an error, try `rustc --explain E0382`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 2 warnings
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer") due to 3 previous errors; 2 warnings emitted
```

### Suggestion

#### Add main.rs to a Canvas tab so to speak

##### `main.rs`

```rust
use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const NOISE_PROFILE_FILE: &str = "noise_profile.txt";

use std::time::{Instant, Duration};

mod live_output; // Import new module
mod bitrate;
mod gui;



// new:

fn start_audio_io() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device found");
    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate().0;
    let buffer_size = 1920;

    let buffer = Arc::new(Mutex::new(vec![0.0f32; buffer_size]));

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                let mut buffer = buffer.lock().unwrap();
                data.copy_from_slice(&buffer[..data.len()]);
            },
            move |err| eprintln!("Stream error: {:?}", err),
            None, // ✅ Fix: Add this argument        
        )
        .unwrap();

    stream.play().unwrap();

    thread::spawn(move || {
        loop {
            {
                let mut buffer = buffer.lock().unwrap();
                for i in 0..buffer_size {
                    buffer[i] = (i as f32 / sample_rate as f32).sin(); // Example: sine wave
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1)); // Keep main thread alive
    }
}




fn main() {
    thread::spawn(|| start_audio_io()); // Run audio processing in background


    // launch_gui(); // Run GUI (Audio Analyzer + Frequency Meter)


    gui::launch_gui();  // Remove if let Err(e)

    eframe::run_native(
        "Audio Analyzer",
        options.clone(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();


    let program_start = Instant::now(); // ✅ Fix: Declare inside main()

    // ✅ Move logging into a separate thread
    std::thread::spawn(move || {
        loop {
            let elapsed = program_start.elapsed().as_secs();
            if elapsed % 5 == 0 {
                println!("⏳ Program Running: {} seconds elapsed.", elapsed);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let device = audio::select_audio_device();
    let config = audio::get_audio_config(&device); // ✅ Define config first

    bitrate::print_audio_bitrate(&config);

    println!("\nUsing input device: {}\n", device.name().unwrap());

    let data = Arc::new(Mutex::new(Vec::new()));
    let note_playing = Arc::new(Mutex::new(false));
    let last_note = Arc::new(Mutex::new("".to_string())); // Track last note

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let note_clone = Arc::clone(&note_playing);
    let last_note_clone = Arc::clone(&last_note);

    let noise_profile = if let Ok(profile) = load_noise_profile() {
        println!("Loaded saved noise profile.");
        profile
    } else {
        println!("Capturing noise profile...");
        let profile = capture_noise_profile(&device, &config);
        save_noise_profile(&profile);
        profile
    };

    // Edited: Ensure display_amplitude() is called live inside input stream processing
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            // before buffer --> do stream analysis
            for &sample in data {
                let amplitude = sample.abs();
                live_output::print_live_amplitude(amplitude); // Call new function    
            }
            // buffer related:
            let mut buffer = data_clone.lock().unwrap();
            buffer.extend_from_slice(data);
            // Begin analysis once buffer has reached 1024 frames (previously 2048)
            static mut PRINT_COUNTER: usize = 0; // Track buffer count

            if buffer.len() >= 1920 {
                unsafe {
                    PRINT_COUNTER += 1;
                    if PRINT_COUNTER % 100 == 0 {  // Print every 10 buffers
                        println!("✅ Processing samples... Buffer size: {}", buffer.len());
                    }
                }
                let buffer_len = buffer.len().min(2048);
                let peaks = fft::analyze_frequencies(&buffer[..buffer_len]);
                
                let mut silence_count = 0;
                let mut total_frames = 0;
                
                let raw_amplitude = buffer.iter().map(|&x| x.abs()).sum::<f32>() / buffer.len() as f32;
                fft::display_amplitude(raw_amplitude, &mut silence_count, &mut total_frames);
            
                analyze_amplitude(&buffer[..buffer_len]); // ✅ Fix applied buffer length 1920 on this device.
            
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

    println!("Capturing noise for 0.5 seconds...");
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("Noise profile captured.");
    
    let buffer = data.lock().unwrap();
    if buffer.len() >= 1920 {
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

    analyze_amplitude(&samples);
}
```

## Out

### Text

Your main.rs is now in a dedicated workspace for further modifications. Let me know the next steps or specific issues you want to address!

### Canvas / main.rs

```rust
use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const NOISE_PROFILE_FILE: &str = "noise_profile.txt";

use std::time::{Instant, Duration};

mod live_output;
mod bitrate;
mod gui;

fn start_audio_io() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device found");
    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate().0;
    let buffer_size = 1920;

    let buffer = Arc::new(Mutex::new(vec![0.0f32; buffer_size]));

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                let mut buffer = buffer.lock().unwrap();
                data.copy_from_slice(&buffer[..data.len()]);
            },
            move |err| eprintln!("Stream error: {:?}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    thread::spawn(move || {
        loop {
            {
                let mut buffer = buffer.lock().unwrap();
                for i in 0..buffer_size {
                    buffer[i] = (i as f32 / sample_rate as f32).sin();
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    thread::spawn(|| start_audio_io());
    gui::launch_gui();
}
```
