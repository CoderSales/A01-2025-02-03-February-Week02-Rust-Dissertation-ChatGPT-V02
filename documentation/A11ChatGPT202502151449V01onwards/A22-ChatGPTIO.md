# Chat

## In

### main.rs

```rust
use midi_freq_analyzer::audio;
use midi_freq_analyzer::fft;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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


    launch_gui(); // Run GUI (Audio Analyzer + Frequency Meter)


    // gui::launch_gui();  // Remove if let Err(e)


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

### gui.rs

```
use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct AudioApp {
    status_message: String,
    log_output: Arc<Mutex<String>>,
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl App for AudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Audio Analyzer");
            ui.separator();

            if ui.button("▶ Record").clicked() {
                self.status_message = "Recording...".to_string();
                
                let log_output = Arc::clone(&self.log_output);
                thread::spawn(move || {
                    let mut log = log_output.lock().unwrap();
                    *log = String::new(); // Reset logs when recording starts

                    for i in 1..=10 {
                        thread::sleep(Duration::from_millis(500));
                        log.push_str(&format!("✅ Processing samples... {}\n", i));
                    }
                });
            }

            if ui.button("⏹ Stop").clicked() {
                self.status_message = "Stopped.".to_string();
            }

            ui.label(&self.status_message);
            ui.separator();

            // Display logs
            let log = self.log_output.lock().unwrap();
            ui.add_sized([400.0, 200.0], egui::TextEdit::multiline(&mut log.clone()));
        });
    }
}

#[derive(Default)]
struct FrequencyMeter {
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl App for FrequencyMeter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚 Frequency Levels");

            let low = *self.low_freq.lock().unwrap();
            let mid = *self.mid_freq.lock().unwrap();
            let high = *self.high_freq.lock().unwrap();

            ui.add(egui::ProgressBar::new(low).show_percentage());
            ui.label("Low Frequencies (20Hz - 250Hz)");

            ui.add(egui::ProgressBar::new(mid).show_percentage());
            ui.label("Mid Frequencies (250Hz - 4kHz)");

            ui.add(egui::ProgressBar::new(high).show_percentage());
            ui.label("High Frequencies (4kHz - 20kHz)");
        });
    }
}

pub fn launch_gui() {
    let options = NativeOptions::default();

    let log_output = Arc::new(Mutex::new(String::new()));
    let low_freq = Arc::new(Mutex::new(0.0));
    let mid_freq = Arc::new(Mutex::new(0.0));
    let high_freq = Arc::new(Mutex::new(0.0));

    let app = AudioApp {
        status_message: "Idle".to_string(),
        log_output: Arc::clone(&log_output),
        low_freq: Arc::clone(&low_freq),
        mid_freq: Arc::clone(&mid_freq),
        high_freq: Arc::clone(&high_freq),
    };

    let freq_meter = FrequencyMeter {
        low_freq,
        mid_freq,
        high_freq,
    };

    // Run GUI1 & GUI2 in separate threads
    thread::spawn(move || {
        eframe::run_native(
            "Audio Analyzer",
            options.clone(),
            Box::new(|_cc| Ok(Box::new(app))),
        )
        .unwrap();
    });

    thread::sleep(Duration::from_millis(100)); // Prevent UI conflicts

    eframe::run_native(
        "Frequency Meter",
        options,
        Box::new(|_cc| Ok(Box::new(freq_meter))),
    )
    .unwrap();
}
```

### fft.rs

```rust
use rustfft::{FftPlanner, num_complex::Complex}; // Old
use std::f32::consts::PI; // Old
use std::sync::{Arc, Mutex}; // New
use std::time::Duration; // Old, Moved
use std::thread; // New

const SAMPLE_RATE: f32 = 44100.0; // Old
const MIN_PEAK_MAGNITUDE: f32 = 5.0; // Old
const MAX_PEAKS: usize = 10; // Old
const FFT_SIZE: usize = 2048; // Old

/// Perform FFT and return raw frequency spectrum + top peaks (V01)
// Edited, Warnings moved out
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32; // old
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();// old

    let raw_amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32; // Old
    // Code removed here, Warnings
    if raw_amplitude < MIN_PEAK_MAGNITUDE { // old, warning print removed
        return vec![]; // old
    }
    // old:
    let hann_window: Vec<f32> = (0..FFT_SIZE)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_SIZE - 1) as f32).cos()))
        .collect();
    // old:
    let windowed_samples: Vec<f32> = centered_samples
        .iter()
        .zip(hann_window.iter())
        .map(|(s, w)| s * w)
        .collect();
    // old:
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(windowed_samples.len());
    // old:
    let mut buffer: Vec<Complex<f32>> = windowed_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);
    // old:
    let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    // old:
    let mut peaks: Vec<(f32, f32)> = vec![];
    // old: (Loop FFT out)
    for (i, &magnitude) in magnitude_spectrum.iter().enumerate().take(FFT_SIZE / 2) {
        let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
        let prev = if i > 0 { magnitude_spectrum[i - 1] } else { 0.0 };
        let next = if i < magnitude_spectrum.len() - 1 { magnitude_spectrum[i + 1] } else { 0.0 };

        if magnitude > prev && magnitude > next && magnitude > MIN_PEAK_MAGNITUDE {
            peaks.push((freq, magnitude));
        }
    }
    // old
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(MAX_PEAKS);
    // Code removed here for magnitude_spectrum. old:
    peaks // Is this instantiating peaks vector?
}
// new, timer:
/// Timer thread that ensures final summary prints after recording
pub fn start_timer(silence_count: Arc<Mutex<usize>>, total_frames: Arc<Mutex<usize>>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(30)); // Simulate program run duration

        let silence = *silence_count.lock().unwrap();
        let total = *total_frames.lock().unwrap();
        let silence_percentage = (silence as f32 / total as f32) * 100.0;

        println!("\n✅ Final Analysis: {}% of the recording was silent.", silence_percentage);
    });
}
// old, edited, last_warning removed:
/// Display amplitude visualization and track silence
pub fn display_amplitude(amplitude: f32, silence_count: &mut usize, total_frames: &mut usize) {
    *total_frames += 1; // Track total frames // New

    let bars = (amplitude * 50.0) as usize;
    let bass = if bars > 30 { "█" } else { " " }; // Edited
    let mids = if bars > 15 { "█" } else { " " }; // Edited
    let treble = if bars > 5 { "█" } else { " " }; // Edited

    // Track silence percentage // New
    if bars == 0 {
        *silence_count += 1;
    }

    // Limit refresh rate to every 50 frames (~0.5s) // Edited
    if *total_frames % 50 == 0 {
        println!("\nBass |{}\nMids |{}\nTreble |{}\n", bass, mids, treble); // Edited for vertical alignment
    }
}
```

There are four other rust files.

Please do not reduce code.

Just note issues.

### CL Output

```bash
$ cargo check
    Checking midi_freq_analyzer v0.1.0 (C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer)
error[E0252]: the name `Duration` is defined multiple times
  --> src/main.rs:15:26
   |
6  | use std::time::Duration;
   |     ------------------- previous import of the type `Duration` here
...
15 | use std::time::{Instant, Duration};
   |                          ^^^^^^^^ `Duration` reimported here
   |
   = note: `Duration` must be defined only once in the type namespace of this module

error[E0425]: cannot find function `launch_gui` in this scope
  --> src/main.rs:72:5
   |
72 |     launch_gui(); // Run GUI (Audio Analyzer + Frequency Meter)
   |     ^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
1  + use crate::gui::launch_gui;
   |

warning: unused import: `Duration`
  --> src/main.rs:15:26
   |
15 | use std::time::{Instant, Duration};
   |                          ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0277]: `(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)` cannot be sent between threads safely
   --> src\gui.rs:101:19
    |
101 |       thread::spawn(move || {
    |  _____-------------_^
    | |     |
    | |     required by a bound introduced by this call
102 | |         eframe::run_native(
103 | |             "Audio Analyzer",
104 | |             options.clone(),
...   |
107 | |         .unwrap();
108 | |     });
    | |_____^ `(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)`, which is required by `{closure@src\gui.rs:101:19: 101:26}: Send`
    = note: required for `Unique<(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)>` to implement `Send`
note: required because it appears within the type `Box<(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)>`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\alloc\src\boxed.rs:235:12
note: required because it appears within the type `Option<Box<(dyn for<'a> FnOnce(&'a mut EventLoopBuilder<UserEvent>) + 'static)>>`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\core\src\option.rs:571:10
note: required because it appears within the type `NativeOptions`
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\eframe-0.31.0\src\epi.rs:279:12
    |
279 | pub struct NativeOptions {
    |            ^^^^^^^^^^^^^
note: required because it's used within this closure
   --> src\gui.rs:101:19
    |
101 |     thread::spawn(move || {
    |                   ^^^^^^^
note: required by a bound in `spawn`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\std\src\thread\mod.rs:672:1
help: use parentheses to call this trait object
    |
108 |     }(/* &mut EventLoopBuilder<UserEvent> */));
    |      ++++++++++++++++++++++++++++++++++++++++

error[E0277]: `(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)` cannot be sent between threads safely
   --> src\gui.rs:101:19
    |
101 |       thread::spawn(move || {
    |  _____-------------_^
    | |     |
    | |     required by a bound introduced by this call
102 | |         eframe::run_native(
103 | |             "Audio Analyzer",
104 | |             options.clone(),
...   |
107 | |         .unwrap();
108 | |     });
    | |_____^ `(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)`, which is required by `{closure@src\gui.rs:101:19: 101:26}: Send`
    = note: required for `Unique<(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)>` to implement `Send`
note: required because it appears within the type `Box<(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)>`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\alloc\src\boxed.rs:235:12
note: required because it appears within the type `Option<Box<(dyn FnOnce(ViewportBuilder) -> ViewportBuilder + 'static)>>`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\core\src\option.rs:571:10
note: required because it appears within the type `NativeOptions`
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\eframe-0.31.0\src\epi.rs:279:12
    |
279 | pub struct NativeOptions {
    |            ^^^^^^^^^^^^^
note: required because it's used within this closure
   --> src\gui.rs:101:19
    |
101 |     thread::spawn(move || {
    |                   ^^^^^^^
note: required by a bound in `spawn`
   --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\std\src\thread\mod.rs:672:1
help: use parentheses to call this trait object
    |
108 |     }(/* ViewportBuilder */));
    |      +++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
   --> src/main.rs:36:10
    |
36  |           .build_output_stream(
    |  __________^^^^^^^^^^^^^^^^^^^-
37  | |             &config.into(),
38  | |             move |data: &mut [f32], _| {
39  | |                 let mut buffer = buffer.lock().unwrap();
...   |
42  | |             move |err| eprintln!("Stream error: {:?}", err),
43  | |         )
    | |_________- argument #4 of type `Option<Duration>` is missing
    |
note: method defined here
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cpal-0.15.3\src\traits.rs:153:8
    |
153 |     fn build_output_stream<T, D, E>(
    |        ^^^^^^^^^^^^^^^^^^^
help: provide the argument
    |
36  ~         .build_output_stream(&config.into(), move |data: &mut [f32], _| {
37  +                 let mut buffer = buffer.lock().unwrap();
38  +                 data.copy_from_slice(&buffer[..data.len()]);
39  +             }, move |err| eprintln!("Stream error: {:?}", err), /* Option<Duration> */)
    |

Some errors have detailed explanations: E0061, E0252, E0277, E0425.
For more information about an error, try `rustc --explain E0061`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 1 warning
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer") due to 5 previous errors; 1 warning emitted
```

### Note

Main two objectives are:

- To keep whatever code is in place, and not to reduce functionality or potential future functionality.

- not to break code even if it means all requirments not satisfied.
