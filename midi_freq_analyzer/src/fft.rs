use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 44100.0;
const MIN_AMPLITUDE: f32 = 0.02; // Ignore low-energy noise
const FFT_SIZE: usize = 2048; // ✅ Ensure correct FFT size

/// Perform FFT and return top frequency peaks
pub fn analyze_frequencies(samples: &[f32]) -> Vec<(f32, f32)> {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let centered_samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Ignore silent frames
    let amplitude = centered_samples.iter().map(|&x| x.abs()).sum::<f32>() / centered_samples.len() as f32;
    if amplitude < MIN_AMPLITUDE {
        return vec![];
    }

    // ✅ Apply Hann window
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
    
    // Extract top 3 frequency peaks
    let mut peak_frequencies: Vec<(f32, f32)> = magnitude_spectrum.iter()
        .enumerate()
        .map(|(i, &m)| ((i as f32) * (SAMPLE_RATE / FFT_SIZE as f32), m)) // ✅ Ensure correct scaling
        .collect();

    peak_frequencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort by magnitude

    peak_frequencies.truncate(3); // ✅ Keep top 3 peaks
    peak_frequencies
}
