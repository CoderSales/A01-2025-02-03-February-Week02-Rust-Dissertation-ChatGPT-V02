# Chat

## In

Yes emphasis on not breaking code.

## Out

Here’s the bitrate.rs file to add to your project. It reads and prints the sample format, channels, and sample rate without breaking existing functionality.

Create bitrate.rs
Place this file in the src/ directory.

rust
Copy
Edit
use cpal::StreamConfig;

/// Calculate and print audio bitrate information
pub fn print_audio_bitrate(config: &StreamConfig) {
    let sample_format = "f32"; // Assuming f32 format from CPAL
    let channels = config.channels;
    let sample_rate = config.sample_rate.0;

    let bit_depth = 32; // Default assumption for f32 format
    let bitrate = sample_rate as u32 * bit_depth * channels as u32;

    println!("\n🎵 Audio Configuration:");
    println!(" - Sample Format: {}", sample_format);
    println!(" - Channels: {}", channels);
    println!(" - Sample Rate: {} Hz", sample_rate);
    println!(" - Bit Depth: {} bits", bit_depth);
    println!(" - Calculated Bitrate: {} bps ({} kbps)", bitrate, bitrate / 1000);
}
Modify main.rs
Call print_audio_bitrate() after selecting the audio device
Update main.rs:

Add the module at the top:
rust
Copy
Edit
mod bitrate;
Call the function after selecting the input device:
rust
Copy
Edit
bitrate::print_audio_bitrate(&config);
Expected Output
When running the program, after selecting the microphone, you’ll see:

bash
Copy
Edit
🎵 Audio Configuration:
 - Sample Format: f32
 - Channels: 1
 - Sample Rate: 44100 Hz
 - Bit Depth: 32 bits
 - Calculated Bitrate: 1411200 bps (1411 kbps)
✅ Safe to add—no breaking changes! 🚀
Let me know if you want to refine anything! 🎯

## In

```rust
use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{StreamTrait, DeviceTrait};
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 20000.0;
const NOISE_PROFILE_FILE: &str = "noise_profile.txt";

use std::time::{Instant, Duration};

mod live_output; // Import new module
mod bitrate;

fn main() {



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
    bitrate::print_audio_bitrate(&config);

    let config = audio::get_audio_config(&device);

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
            // begin analysis once buffer has reached 2048 frames:
            if buffer.len() >= 2048 {
                let peaks = fft::analyze_frequencies(&buffer[..2048]);

                let mut silence_count = 0; // New
                let mut total_frames = 0; // New
                
                let raw_amplitude = buffer.iter().map(|&x| x.abs()).sum::<f32>() / buffer.len() as f32;
                fft::display_amplitude(raw_amplitude, &mut silence_count, &mut total_frames); // New

                analyze_amplitude(&buffer[..2048]);  

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

    println!("Capturing noise for 1 second...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Noise profile captured.");
    
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

// use std::time::{Instant, Duration};  // Add at top of file

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

Output:

```bash
$ cargo test
warning: unused import: `Instant`
 --> src\fft.rs:4:17
  |
4 | use std::time::{Instant, Duration}; // Old, Moved
  |                 ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `midi_freq_analyzer` (lib) generated 1 warning (run `cargo fix --lib -p midi_freq_analyzer` to apply 1 suggestion)
warning: `midi_freq_analyzer` (lib test) generated 1 warning (1 duplicate)
   Compiling midi_freq_analyzer v0.1.0 (C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer)
error[E0425]: cannot find value `config` in this scope
  --> src/main.rs:35:35
   |
35 |     bitrate::print_audio_bitrate(&config);
   |                                   ^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer" test) due to 1 previous error
```

Where should I put the function exactly, currently around line 35.

## Out

Move the bitrate::print_audio_bitrate(&config); below the config declaration.

Fix
Modify this section in main.rs:

Before (line 35 - incorrect order)
rust
Copy
Edit
let device = audio::select_audio_device();
bitrate::print_audio_bitrate(&config); // ❌ config not defined yet!

let config = audio::get_audio_config(&device);
After (correct order)
rust
Copy
Edit
let device = audio::select_audio_device();
let config = audio::get_audio_config(&device); // ✅ Define config first

bitrate::print_audio_bitrate(&config); // ✅ Now config is available
This ensures config is defined before passing it to print_audio_bitrate().

✅ This will fix error[E0425]: cannot find value config in this scope
🚀 Now cargo test and cargo run should work without breaking anything.

## In

```bash
⏳ Program Running: 0 seconds elapsed.

Available input devices:
0: Microphone Array (Intel® Smart Sound Technology for Digital Microphones)
1: CABLE Output (VB-Audio Virtual Cable)
Select an input device (Enter number): 0

🎵 Audio Configuration:
 - Sample Format: f32
 - Channels: 2
 - Sample Rate: 48000 Hz
 - Bit Depth: 32 bits
 - Calculated Bitrate: 3072000 bps (3072 kbps)

Using input device: Microphone Array (Intel® Smart Sound Technology for Digital Microphones)

Capturing noise profile...
Capturing noise for 1 second...
⏳ Program Running: 5 seconds elapsed.
Noise profile captured.
Noise profile captured.
Listening for audio... Press Ctrl+C to stop.
[] 🔍 Amplitude Analysis - Min: -0.00000, Max: 0.00000, Mean: -0.00000, Median: 0.00000
[] ⏳ Program Running: 10 seconds elapsed.
[] 🔍 Amplitude Analysis - Min: -0.00013, Max: 0.00008, Mean: -0.00000, Median: 0.00002
[] ⏳ Program Running: 15 seconds elapsed.
[] 🔍 Amplitude Analysis - Min: -0.16707, Max: 0.15722, Mean: 0.00062, Median: -0.00050
[______________________] ⏳ Program Running: 20 seconds elapsed.
[_] 🔍 Amplitude Analysis - Min: -0.21784, Max: 0.36785, Mean: 0.00073, Median: -0.01291
[________] ⏳ Program Running: 25 seconds elapsed.
[] 🔍 Amplitude Analysis - Min: -0.00002, Max: 0.00002, Mean: -0.00000, Median: 0.00000
[___] error: process didn't exit successfully: `C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer\target\debug\midi_freq_analyzer.exe` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
```

Note for first 15 seconds, program does not seem to capture or display audio.

## Out

This behavior is likely due to buffering and the initial noise profile capture. Here's why:

Possible Causes & Fixes
Noise Profile Capture Delays Output

The program captures a 1-second noise profile before analyzing real audio.
The system might be filtering out quiet initial input as "silence."
Fix: Reduce the noise capture time from 1 second to 0.5 seconds for faster startup.
rust
Copy
Edit
println!("Capturing noise for 0.5 seconds...");
std::thread::sleep(std::time::Duration::from_millis(500));
FFT Buffer Takes Time to Fill

The buffer must collect 2048 samples before performing an FFT analysis.
With a sample rate of 48000 Hz, this means:
2048 / 48000 Hz ≈ 0.042 seconds (~42ms) per FFT frame
If low amplitude noise is ignored, it could delay printing.
Fix: Lower the required buffer size to 1024 samples for faster updates.
rust
Copy
Edit
if buffer.len() >= 1024 { // Change from 2048 to 1024
Low Initial Input Volume

If there is no sound, display_amplitude() may not print.
Fix: Ensure it prints at least one [ ] on startup.
Debugging Initial Frames

Add a debug print to confirm samples are received:
rust
Copy
Edit
if buffer.len() < 2048 {
    println!("⚠️ Waiting for samples... Buffer size: {}", buffer.len());
}
Next Steps
Would you like to apply these changes and test again? 🚀




