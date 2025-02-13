# ChatGPT

## Input

### audio.rs

#### original (working)

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{StreamConfig, Device};

/// Selects the best available input device
pub fn get_audio_device() -> Device {
    let host = cpal::default_host();

    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select VB-Audio Virtual Cable if available, otherwise default input device
    host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device())
        .expect("No suitable audio input device found")
}

/// Retrieves default audio config
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}


#### modified (not working)

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{StreamConfig, Device};

use std::io;
use cpal::{Device, HostTrait};

pub fn select_audio_device() -> Device {
    let host = cpal::default_host();
    let devices: Vec<_> = host.input_devices().unwrap().collect();

    println!("\nAvailable input devices:");
    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("Unknown".to_string()));
    }

    print!("Select an input device (Enter number): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap_or(0);

    devices.get(index).expect("Invalid selection, using default.").clone()
}

/// Selects the best available input device
pub fn get_audio_device() -> Device {
    let host = cpal::default_host();

    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select VB-Audio Virtual Cable if available, otherwise default input device
    host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device())
        .expect("No suitable audio input device found")
}

/// Retrieves default audio config
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}

#### cargo run output

```bash
error[E0252]: the name `Device` is defined multiple times
 --> src\audio.rs:5:12
  |
2 | use cpal::{StreamConfig, Device};
  |                          ------ previous import of the type `Device` here
...
5 | use cpal::{Device, HostTrait};
  |            ^^^^^^--
  |            |
  |            `Device` reimported here
  |            help: remove unnecessary import
  |
  = note: `Device` must be defined only once in the type namespace of this module

error[E0432]: unresolved import `cpal::HostTrait`
 --> src\audio.rs:5:20
  |
5 | use cpal::{Device, HostTrait};
  |                    ^^^^^^^^^ no `HostTrait` in the root
  |
  = help: consider importing this trait instead:
          cpal::traits::HostTrait

warning: unused import: `Device`
 --> src\audio.rs:5:12
  |
5 | use cpal::{Device, HostTrait};
  |            ^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

Some errors have detailed explanations: E0252, E0432.
For more information about an error, try `rustc --explain E0252`.
warning: `midi_freq_analyzer` (lib) generated 1 warning
error: could not compile `midi_freq_analyzer` (lib) due to 2 previous errors; 1 warning emitted

steph@Laptop7JA MINGW64 ~/OneDrive/Documents/48-Rust/A06ChatGPT/A01-proj/A03Project01/A01-proj/A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT/midi_freq_analyzer/src (main)
$ cargo run
   Compiling midi_freq_analyzer v0.1.0 (C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer)
error[E0252]: the name `HostTrait` is defined multiple times
 --> src\audio.rs:1:52
  |
1 | use cpal::traits::{DeviceTrait, HostTrait, Device, HostTrait};
  |                                 ---------          ^^^^^^^^^ `HostTrait` reimported here
  |                                 |
  |                                 previous import of the trait `HostTrait` here
  |
  = note: `HostTrait` must be defined only once in the type namespace of this module

error[E0432]: unresolved import `cpal::traits::Device`
 --> src\audio.rs:1:44
  |
1 | use cpal::traits::{DeviceTrait, HostTrait, Device, HostTrait};
  |                                            ^^^^^^ no `Device` in `traits`
  |
  = help: consider importing this struct instead:
          cpal::Device

warning: unused import: `HostTrait`
 --> src\audio.rs:1:52
  |
1 | use cpal::traits::{DeviceTrait, HostTrait, Device, HostTrait};
  |                                                    ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

Some errors have detailed explanations: E0252, E0432.
For more information about an error, try `rustc --explain E0252`.
warning: `midi_freq_analyzer` (lib) generated 1 warning
error: could not compile `midi_freq_analyzer` (lib) due to 2 previous errors; 1 warning emitted
```

### main.rs

#### Modified

Replacing the follwing line

```rust
    let device = audio::get_audio_device();
```

with this line

```rust
    let device = select_audio_device();
```

gave this file, which may or may not have contributed to the issue:

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
```

### Step 2 onward not addressed

### Elements of last ChatGPT Output not addressed

✅ 2. Detect Two Notes Simultaneously (Intervals)
🔹 Where?
Modify FFT analysis function to extract two strongest frequencies.

🔧 Changes to main.rs
Modify this part inside move |data: &[f32], _: &_|:

rust
Copy
Edit
let peaks = fft::analyze_frequencies(&buffer[..2048]);
Change it to:

rust
Copy
Edit
if peaks.len() >= 2 {
    let fundamental = peaks[0].0;
    let second_note = peaks[1].0; // Second strongest peak
Then modify the print statement:

rust
Copy
Edit
println!("Detected Notes: {:.2} Hz ({}) & {:.2} Hz ({})", 
    fundamental, frequency_to_note(fundamental), 
    second_note, frequency_to_note(second_note)
);
🔹 Effect?
Now two notes (intervals, chords) will be detected!

✅ 3. Limit Recording to 2 Seconds
🔹 Where?
Modify main.rs inside stream.play()

🔧 Change This in main.rs
Replace:

rust
Copy
Edit
println!("Listening for audio... Press Ctrl+C to stop.");
std::thread::sleep(std::time::Duration::from_secs(30));
With:

rust
Copy
Edit
println!("Recording for 2 seconds...");
std::thread::sleep(std::time::Duration::from_secs(2));
println!("Recording stopped.");
🔹 Effect?
The program stops recording after 2 seconds automatically.

✅ 4. Load Noise Profile If Present
🔹 Where?
Modify main.rs where load_noise_profile() is called.

🔧 Changes to main.rs
Replace:

rust
Copy
Edit
let noise_profile = if let Ok(profile) = load_noise_profile() {
With:

rust
Copy
Edit
use std::path::Path;
let noise_profile = if Path::new(NOISE_PROFILE_FILE).exists() {
🔹 Effect?
If noise_profile.txt exists, it loads automatically without recapturing.

📌 Summary of Changes
✅ User selects an audio device at startup
✅ Detects two strongest frequencies (intervals/chords)
✅ Stops recording after 2 seconds
✅ Automatically loads noise profile if available

🚀 Next Steps
🔹 Test each feature separately
🔹 Let me know if anything breaks!
🔹 Future: Add real-time frequency visualization

💡 Let me know if you need further refinements! 🚀

### Priority

Fix file ensuring it compiles and runs (warnings are fine)

Err on the side of reverting to file before these changes added 
