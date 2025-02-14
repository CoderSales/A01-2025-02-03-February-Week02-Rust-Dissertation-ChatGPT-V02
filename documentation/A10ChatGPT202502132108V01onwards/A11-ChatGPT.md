# ChatGPT

## Output

Yes, E0425 is caused because LAST_WARNING was used inside unsafe without being declared as a static mutable variable. Since Rust enforces strict safety rules, it prevents the use of undeclared global mutable variables.

🔧 Fix Without unsafe
Instead of using static mut, use a thread-safe solution with std::sync::Mutex<bool> inside lazy_static!.**

✅ Solution: Use lazy_static! to Avoid unsafe
rust
Copy
Edit
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;
use std::sync::Mutex;
use lazy_static::lazy_static;

const SAMPLE_RATE: f32 = 44100.0;
const MIN_PEAK_MAGNITUDE: f32 = 5.0;  // Ignore weak frequencies
const MAX_PEAKS: usize = 10;          // Limit detected peaks
const FFT_SIZE: usize = 2048;

lazy_static! {
    static ref LAST_WARNING: Mutex<bool> = Mutex::new(false);
}

// 1️⃣ Compute Amplitude (No excessive debug prints)
fn compute_amplitude(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();
    centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32
}

// 2️⃣ Apply Hann Window (Unchanged)
fn apply_hann_window(samples: &[f32]) -> Vec<f32> {
    let hann_window: Vec<f32> = (0..samples.len())
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (samples.len() - 1) as f32).cos()))
        .collect();
    samples.iter().zip(hann_window.iter()).map(|(s, w)| s * w).collect()
}

// 3️⃣ Compute FFT (Unchanged)
fn compute_fft(samples: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);
    buffer.iter().map(|c| c.norm()).collect()
}

// 4️⃣ Detect Peaks (Unchanged)
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

// 5️⃣ Main Analyze Function (Now Avoids `unsafe`)
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    if samples.is_empty() {
        println!("⚠️ No input audio detected.");
        return vec![];
    }

    let amplitude = compute_amplitude(samples);

    let mut last_warning = LAST_WARNING.lock().unwrap();

    if amplitude < MIN_PEAK_MAGNITUDE {
        if !*last_warning {
            println!("⚠️ No significant sound detected! Try increasing volume or checking microphone input.");
            *last_warning = true;
        }
        return vec![];
    } else {
        *last_warning = false;  // Reset warning state when sound detected
    }

    let windowed_samples = apply_hann_window(samples);
    let magnitude_spectrum = compute_fft(&windowed_samples);
    let peaks = detect_peaks(&magnitude_spectrum);

    peaks
}

// 6️⃣ Display Amplitude Bars (Fix Missing `_` Printing)
fn display_amplitude(amplitude: f32) {
    let bars = (amplitude * 50.0) as usize;
    if bars > 0 {
        println!("{}", "_".repeat(bars));
    }
}
🔍 Key Fixes & Benefits
✔ Removes unsafe
✔ Uses lazy_static! + Mutex<bool> for thread-safe state management
✔ Only prints "No Sound" once per silent period
✔ Ensures _ bars display properly when sound is detected

📌 Next Steps
1️⃣ Run cargo run again.
2️⃣ Check if _ bars print when sound is detected.
3️⃣ Confirm "No sound" warning only prints once per silence event.

This should fix E0425 while keeping the code safe. 🚀🔊 Let me know how it runs!
