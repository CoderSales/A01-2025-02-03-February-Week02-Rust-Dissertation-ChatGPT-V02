## ChatGPT

## Input: Review Verbose output and Code

### cargo run command line out

🔍 Debug: First 10 Samples = [1.2199749e-5, 1.220962e-5, 1.3411506e-5, 1.3423456e-5, 1.2772883e-5, 1.2771004e-5, 9.645828e-6, 9.662715e-6, 4.364336e-6, 4.3693717e-6]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [-1.43969455e-5, -1.4401981e-5, -6.4667224e-6, -6.4612864e-6, 3.0110427e-6, 3.0112428e-6, 1.0558518e-5, 1.0567111e-5, 1.40955135e-5, 1.4087021e-5]
🔍 Debug: Amplitude Computed = 0.00002
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [1.903983e-5, 1.9029852e-5, 2.1553076e-5, 2.1543e-5, 2.3108392e-5, 2.3108292e-5, 2.3988954e-5, 2.3993893e-5, 2.4148694e-5, 2.4140396e-5]
🔍 Debug: Amplitude Computed = 0.00002
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [-3.592009e-5, -3.59114e-5, -3.5757603e-5, -3.5745652e-5, -3.2681586e-5, -3.2684944e-5, -2.6882864e-5, -2.6886524e-5, -1.9387406e-5, -1.9390865e-5]
🔍 Debug: Amplitude Computed = 0.00002
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [-1.7652173e-5, -1.7650094e-5, -2.2670525e-5, -2.2668946e-5, -2.6253965e-5, -2.6262564e-5, -2.9350253e-5, -2.935371e-5, -3.2494747e-5, -3.2499884e-5]
🔍 Debug: Amplitude Computed = 0.00002
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [1.511568e-5, 1.511558e-5, 1.735852e-5, 1.7363958e-5, 1.9676285e-5, 1.9685083e-5, 1.8523973e-5, 1.8520715e-5, 1.2863657e-5, 1.28598995e-5]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [1.6971024e-5, 1.6970924e-5, 9.848859e-6, 9.845401e-6, 1.9200536e-6, 1.9183747e-6, -6.072953e-6, -6.0749317e-6, -1.3062973e-5, -1.30564595e-5]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [-7.2294495e-7, -7.2116615e-7, -5.6445197e-6, -5.634548e-6, -1.2183417e-5, -1.2183617e-5, -1.8587069e-5, -1.8587169e-5, -2.3131639e-5, -2.3131639e-5]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [2.1163373e-6, 2.1041853e-6, 1.1762711e-6, 1.1605613e-6, -3.6243694e-6, -3.6308845e-6, -8.120317e-6, -8.120217e-6, -9.0018375e-6, -9.0018375e-6]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
🔍 Debug: First 10 Samples = [-2.5186116e-6, -2.5084385e-6, -7.0726473e-6, -7.071169e-6, -8.4729445e-6, -8.486373e-6, -4.8338165e-6, -4.8389525e-6, 2.5277905e-6, 2.544779e-6]
🔍 Debug: Amplitude Computed = 0.00001
⚠️ No significant sound detected! Try increasing volume or checking microphone input.

#### Evaluation

verbose

no significant sound warning verbose

no `_` bars for visualization

###  Priority

stop print outs while no mic

just print no significant mic sound once in output


#### Analyze code

look for one imporvement to make

### Code

#### main.rs

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
    let device = audio::select_audio_device();
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

    println!("Capturing noise for 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));
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

fn analyze_amplitude(samples: &[f32]) {
    if samples.is_empty() {
        println!("No audio recorded.");
        return;
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

#### audio.rs

```rust
use cpal::traits::{DeviceTrait, HostTrait}; // ✅ Fix import
use cpal::{StreamConfig, Device};
use std::io;

/// **User selects an input device at startup**
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

    devices.get(index).cloned().expect("Invalid selection, using default.")
}


/// **Gets default input device if user skips selection**
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

/// **Retrieves default audio config**
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}
```

#### fft.rs

```rust
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 44100.0;
const MIN_PEAK_MAGNITUDE: f32 = 5.0;  // Ignore weak frequencies
const MAX_PEAKS: usize = 10;           // Limit detected peaks


const FFT_SIZE: usize = 2048;

// 1️⃣ Extract Amplitude Calculation
// Move amplitude analysis to its own function.

fn compute_amplitude(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        println!("⚠️ No audio samples received!");
        return 0.0;
    }

    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();
    let amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;

    // Debugging Output
    println!("🔍 Debug: Amplitude Computed = {:.5}", amplitude);

    amplitude
}

// 2️⃣ Extract Windowing Function
// Move Hann window application into a separate function.

fn apply_hann_window(samples: &[f32]) -> Vec<f32> {
    let hann_window: Vec<f32> = (0..samples.len())
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (samples.len() - 1) as f32).cos()))
        .collect();

    samples.iter().zip(hann_window.iter()).map(|(s, w)| s * w).collect()
}

// 3️⃣ Extract FFT Processing
// Move FFT computation to a function.

fn compute_fft(samples: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    buffer.iter().map(|c| c.norm()).collect()
}

// 4️⃣ Extract Peak Detection
// Move peak detection logic into a function.

fn detect_peaks(magnitude_spectrum: &[f32]) -> Vec<(f32, f32)> {
    let mut peaks: Vec<(f32, f32)> = vec![];

    for i in 1..magnitude_spectrum.len() - 1 {
        let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
        let prev = magnitude_spectrum[i - 1];
        let curr = magnitude_spectrum[i];
        let next = magnitude_spectrum[i + 1];

        if curr > prev && curr > next && curr > MIN_PEAK_MAGNITUDE {
            peaks.push((freq, curr));
        }
    }

    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(MAX_PEAKS);

    peaks
}

// 5️⃣ Refactored analyze_frequencies()
// Now, the primary analyze_frequencies() function is much cleaner.

/// Perform FFT and return raw frequency spectrum + top peaks (V01)
/// Identify dominant frequency peaks (V02)

pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    if samples.is_empty() {
        println!("⚠️ No input audio detected.");
        return vec![];
    }

    // Print first 10 samples for debugging
    println!("🔍 Debug: First 10 Samples = {:?}", &samples.iter().take(10).collect::<Vec<&f32>>());

    let amplitude = compute_amplitude(samples);

    if amplitude < MIN_PEAK_MAGNITUDE {
        println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
        return vec![];
    }

    let windowed_samples = apply_hann_window(samples);
    let magnitude_spectrum = compute_fft(&windowed_samples);
    let peaks = detect_peaks(&magnitude_spectrum);

    peaks
}


// pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
//     let mean = samples.iter().sum::<f32>() / samples.len() as f32;
//     let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

//     // Compute raw amplitude (before FFT)
//     let raw_amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;

//     // Debug print to check if microphone is capturing sound
//     // println!("Raw Amplitude: {:.5}", amplitude);
//     // println!("Raw Amplitude: {:.5}", display_amplitude(raw_amplitude));
    
//     // raw_amplitude = amplitude;
//     // display_amplitude(raw_amplitude);

//     // Integration
//     // Modify where display_amplitude() is called to pass an iteration counter:
//     let mut count = 0;
//     let mut last_warning = false;
//     let mut frame = 0;

//     for sample in samples {
//         display_amplitude(*sample, &mut count, &mut last_warning, frame);
//         frame += 1;
//         count += 1;
//     }


//     if raw_amplitude < MIN_PEAK_MAGNITUDE {
//         println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
//         return vec![];
//     }    

//     let hann_window: Vec<f32> = (0..FFT_SIZE)
//         .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_SIZE - 1) as f32).cos()))
//         .collect();

//     let windowed_samples: Vec<f32> = centered_samples
//         .iter()
//         .zip(hann_window.iter())
//         .map(|(s, w)| s * w)
//         .collect();

//     let mut planner = FftPlanner::new();
//     let fft = planner.plan_fft_forward(windowed_samples.len());

//     let mut buffer: Vec<Complex<f32>> = windowed_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
//     fft.process(&mut buffer);

//     let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    
//     // (V02)
//     let mut peaks: Vec<(f32, f32)> = vec![];

//     // Loop through FFT output
//     for (i, &magnitude) in magnitude_spectrum.iter().enumerate().take(FFT_SIZE / 2) {
//         let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
//         // println!("Freq: {:.2} Hz, Magnitude: {:.5}", freq, magnitude);
        
//         // (V02)
//         let prev = if i > 0 { magnitude_spectrum[i - 1] } else { 0.0 };
//         let next = if i < magnitude_spectrum.len() - 1 { magnitude_spectrum[i + 1] } else { 0.0 };

//         // Peak must be a local max and exceed threshold
//         if magnitude > prev && magnitude > next && magnitude > MIN_PEAK_MAGNITUDE {
//             peaks.push((freq, magnitude));
//         }
//     }

//     // (V02)
//     // Sort peaks by magnitude (strongest first) and limit count
//     peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//     peaks.truncate(MAX_PEAKS);
    
//     // Reintroduce if V02 works:
//     // ✅ Detect Peaks (local max with log spacing)
//     let mut peak_frequencies: Vec<(f32, f32)> = vec![];

//     for i in 1..magnitude_spectrum.len() - 1 {
//         let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
//         let prev = magnitude_spectrum[i - 1];
//         let curr = magnitude_spectrum[i];
//         let next = magnitude_spectrum[i + 1];

//         // Log-scale peak detection
//         if curr > prev && curr > next {
//             peak_frequencies.push((freq, curr));
//         }
//     }

//     peak_frequencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort by magnitude
//     peak_frequencies.truncate(3); // Keep top 3 peaks

//     peak_frequencies;


//     // (V02)
//     // ✅ **Print only filtered dominant peaks**
//     for &(freq, magnitude) in &peaks {
//         println!("Peak Freq: {:.2} Hz, Magnitude: {:.2}", freq, magnitude);
//     }

//     peaks
// }

fn display_amplitude(amplitude: f32, count: &mut usize, last_warning: &mut bool, frame: usize) {
    let bars = (amplitude * 50.0) as usize;  // Scale output
    let visual = "_".repeat(bars);

    if frame % 10 == 0 {  // Reduce print frequency (every 10 frames)
    //     if !visual.is_empty() {
    //         println!("{}", visual);
    //         *count = 0;
    //         *last_warning = false;
    //     } else {
    //         *count += 1;
    //     }
    // }
        if bars > 0 {
            println!("{}", visual);
            *count = 0;
            *last_warning = false;
        } else {
            *count += 1;
        }
    }

    // Show warning only if there was NO sound for X iterations
    if *count >= 20 && !*last_warning {
        println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
        *last_warning = true;
    }
}
```

#### lib.rs

```rust
pub mod audio;
pub mod fft;
```
