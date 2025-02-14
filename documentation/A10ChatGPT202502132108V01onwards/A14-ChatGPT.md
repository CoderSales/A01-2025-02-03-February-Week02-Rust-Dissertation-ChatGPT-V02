# ChatGPT

## Out

Incremental Fix: Remove Mutable Static Reference & Maintain Functionality
🔹 Change One Line to Fix LAST_WARNING_TIME Without static mut
🚀 Solution
Replace:

rust
Copy
Edit
static mut LAST_WARNING_TIME: Option<Instant> = None; // Tracks last warning time
With:

rust
Copy
Edit
let mut last_warning_time: Option<Instant> = None;
✅ Explanation:
Fixes the static mut issue by moving last_warning_time into analyze_frequencies(), keeping it local.
Prevents undefined behavior from multiple mutable references to a static variable.
Still limits the warning message to once every 10 seconds.
📌 Next Steps
1️⃣ Compile (cargo build) → Should now work without static mut issues.
2️⃣ Run (cargo run) → Verify warning appears at most once per 10 seconds.

This is the smallest working change to fix the issue while keeping functionality intact. 🚀

## In

### Replacement refers to code not in current fft.rs nor seemingly in codebase

```bash
Replace:

rust
Copy
Edit
static mut LAST_WARNING_TIME: Option<Instant> = None; // Tracks last warning time
With:

rust
Copy
Edit
let mut last_warning_time: Option<Instant> = None;

```


### Current fft.rs

```rust
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 44100.0;
const MIN_PEAK_MAGNITUDE: f32 = 5.0;  // Ignore weak frequencies
const MAX_PEAKS: usize = 10;           // Limit detected peaks


const FFT_SIZE: usize = 2048;

use std::time::{Instant, Duration};


/// Perform FFT and return raw frequency spectrum + top peaks (V01)
/// Identify dominant frequency peaks (V02)
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    let mut last_warning_time = Instant::now();
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Compute raw amplitude (before FFT)
    let raw_amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;

    // Debug print to check if microphone is capturing sound
    // println!("Raw Amplitude: {:.5}", amplitude);
    // println!("Raw Amplitude: {:.5}", display_amplitude(raw_amplitude));
    
    // raw_amplitude = amplitude;
    // display_amplitude(raw_amplitude);

    // Integration
    // Modify where display_amplitude() is called to pass an iteration counter:
    let mut count = 0;
    let mut last_warning = false;
    let mut frame = 0;

    for sample in samples {
        display_amplitude(*sample, &mut count, &mut last_warning, frame, &mut last_warning_time);
        frame += 1;
        count += 1;
    }


    if raw_amplitude < MIN_PEAK_MAGNITUDE {
        println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
        return vec![];
    }    

    let hann_window: Vec<f32> = (0..FFT_SIZE)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_SIZE - 1) as f32).cos()))
        .collect();

    let windowed_samples: Vec<f32> = centered_samples
        .iter()
        .zip(hann_window.iter())
        .map(|(s, w)| s * w)
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(windowed_samples.len());

    let mut buffer: Vec<Complex<f32>> = windowed_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);

    let magnitude_spectrum: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    
    // (V02)
    let mut peaks: Vec<(f32, f32)> = vec![];

    // Loop through FFT output
    for (i, &magnitude) in magnitude_spectrum.iter().enumerate().take(FFT_SIZE / 2) {
        let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
        // println!("Freq: {:.2} Hz, Magnitude: {:.5}", freq, magnitude);
        
        // (V02)
        let prev = if i > 0 { magnitude_spectrum[i - 1] } else { 0.0 };
        let next = if i < magnitude_spectrum.len() - 1 { magnitude_spectrum[i + 1] } else { 0.0 };

        // Peak must be a local max and exceed threshold
        if magnitude > prev && magnitude > next && magnitude > MIN_PEAK_MAGNITUDE {
            peaks.push((freq, magnitude));
        }
    }

    // (V02)
    // Sort peaks by magnitude (strongest first) and limit count
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(MAX_PEAKS);
    
    // Reintroduce if V02 works:
    // ✅ Detect Peaks (local max with log spacing)
    let mut peak_frequencies: Vec<(f32, f32)> = vec![];

    for i in 1..magnitude_spectrum.len() - 1 {
        let freq = (i as f32) * (SAMPLE_RATE / FFT_SIZE as f32);
        let prev = magnitude_spectrum[i - 1];
        let curr = magnitude_spectrum[i];
        let next = magnitude_spectrum[i + 1];

        // Log-scale peak detection
        if curr > prev && curr > next {
            peak_frequencies.push((freq, curr));
        }
    }

    peak_frequencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort by magnitude
    peak_frequencies.truncate(3); // Keep top 3 peaks

    peak_frequencies;


    // (V02)
    // ✅ **Print only filtered dominant peaks**
    for &(freq, magnitude) in &peaks {
        println!("Peak Freq: {:.2} Hz, Magnitude: {:.2}", freq, magnitude);
    }

    peaks
}

fn display_amplitude(amplitude: f32, count: &mut usize, last_warning: &mut bool, frame: usize, last_warning_time: &mut Instant) {
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

    // Show warning only **once every 10 seconds** if continuous silence
    if *count >= 20 && !*last_warning {
        let now = Instant::now();
        if now.duration_since(*last_warning_time) >= Duration::from_secs(1000) {
            println!("\n⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
            *last_warning = true;
            *last_warning_time = now; // Update last warning time
        }
    }
}

```


### Priority

One line change from current fft.rs as shown here

## Out

Change this line:

rust
Copy
Edit
if now.duration_since(*last_warning_time) >= Duration::from_secs(1000) {
To:

rust
Copy
Edit
if now.duration_since(*last_warning_time) >= Duration::from_secs(10) {
Why?
Fixes incorrect 1000 seconds (16+ minutes) to the intended 10 seconds delay.
Keeps functionality unchanged.
Minimal impact—only one line edited. 🚀

## In

redo

stick with current fft.rs

trying to increase delay

even though 1000 seems high, 

in actuality, 

warning printed

rapidly multiple times in less than a second when no sound.

Priority: one line change.

Priority: decrease frequency of no sound warning.

Priority: Try to avoid error.

Priority: incremental change.
