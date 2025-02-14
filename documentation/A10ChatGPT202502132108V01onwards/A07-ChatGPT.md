# Chat

## Output: Refactor analyze frequencies method

✅ Step 11: Refactor analyze_frequencies() for Simplicity
🔹 Fix: Split analyze_frequencies() into Smaller Functions
Goal: Improve readability, maintainability, and reduce verbosity.

1️⃣ Extract Amplitude Calculation
Move amplitude analysis to its own function.

rust
Copy
Edit
fn compute_amplitude(samples: &[f32]) -> f32 {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();
    centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32
}
2️⃣ Extract Windowing Function
Move Hann window application into a separate function.

rust
Copy
Edit
fn apply_hann_window(samples: &[f32]) -> Vec<f32> {
    let hann_window: Vec<f32> = (0..samples.len())
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (samples.len() - 1) as f32).cos()))
        .collect();

    samples.iter().zip(hann_window.iter()).map(|(s, w)| s * w).collect()
}
3️⃣ Extract FFT Processing
Move FFT computation to a function.

rust
Copy
Edit
fn compute_fft(samples: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    buffer.iter().map(|c| c.norm()).collect()
}
4️⃣ Extract Peak Detection
Move peak detection logic into a function.

rust
Copy
Edit
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
5️⃣ Refactored analyze_frequencies()
Now, the main function is much cleaner.

rust
Copy
Edit
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
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
🚀 Next Steps
1️⃣ Test Output - Ensure reduced verbosity.
2️⃣ Check Performance - Functions should improve maintainability without slowdowns.
3️⃣ Fine-Tune Thresholds - Adjust MIN_PEAK_MAGNITUDE if warnings are too frequent.

Let me know if you want further optimizations! 🚀🎛

## Input: Not committed analyze_frequencies method refactor Bug no amplitude detected Verbose Warning

### The following code produced no sound output amplitude underscores

Note that commented code was working before new code was added.

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
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();
    centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32
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
        if !visual.is_empty() {
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

Changes to fft.rs

not committed

as code not performing correctly.

As mic even when mic has sound in,

Only warning shows continuously.

### cargo run output

⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.
⚠️ No significant sound detected! Try increasing volume or checking microphone input.

## Output: Bug Fix No Sound Amplitude Detected

🚨 Bug Fix: No Sound Amplitude Detected
Issue: Only warning messages appear, even when the microphone has sound.
Cause: compute_amplitude(samples) may always return 0, which triggers the warning.

🛠 Fix: Debug Compute Amplitude
🔹 Step 1: Print Amplitude Values
Modify compute_amplitude() to print values and verify if amplitude is calculated correctly.

```rust
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

```

🔹 Step 2: Debug Display Function
Ensure amplitude visualization works.

```rust
fn display_amplitude(amplitude: f32, count: &mut usize, last_warning: &mut bool, frame: usize) {
    let bars = (amplitude * 50.0) as usize;  // Scale output
    let visual = "_".repeat(bars);

    if frame % 10 == 0 {  // Reduce print frequency (every 10 frames)
        if bars > 0 {
            println!("{}", visual);
            *count = 0;
            *last_warning = false;
        } else {
            *count += 1;
        }
    }

    if *count >= 20 && !*last_warning {
        println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
        *last_warning = true;
    }
}

```

🔹 Step 3: Print Raw Samples
Modify analyze_frequencies() to print first few samples and confirm they are not all zero.

```rust
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
```

✅ Expected Outcome After Fix
1️⃣ If audio is being detected, debug messages should show:

First few samples
Computed amplitude
_ bars for visualization
2️⃣ If samples are all zero, there may be an issue with microphone input capture.

🚀 Next Steps
1️⃣ Run cargo run and check debugging output.
2️⃣ If amplitude is still zero, check the microphone settings.
3️⃣ If amplitude prints but no _ bars appear, adjust display_amplitude().

Let me know if the issue persists! 🚀🎛
