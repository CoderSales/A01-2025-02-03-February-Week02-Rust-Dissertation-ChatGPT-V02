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
