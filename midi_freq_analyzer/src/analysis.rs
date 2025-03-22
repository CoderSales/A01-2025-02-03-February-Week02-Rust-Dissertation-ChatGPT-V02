use std::time::{Instant, Duration};


pub fn analyze_amplitude(samples: &[f32]) {
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

}
