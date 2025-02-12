use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use hound;
use rodio::{Decoder, OutputStream, source::Source};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use std::fs::File;
use std::io::BufReader;

const CHUNK_SIZE: usize = 256;
const DOWNSAMPLE_FACTOR: usize = 8;
const FPS: usize = 240;

pub struct AudioVisualizer {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub is_playing: Arc<Mutex<bool>>,
}

impl AudioVisualizer {
    pub fn new(audio_duration_secs: f64) -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE / 2]));
        let is_playing = Arc::new(Mutex::new(true));

        let waveform_clone = Arc::clone(&waveform);
        let fft_result_clone = Arc::clone(&fft_result);
        let is_playing_clone = Arc::clone(&is_playing);

        let total_frames = (audio_duration_secs * FPS as f64) as usize;
        let time_per_frame = audio_duration_secs / total_frames as f64;

        thread::spawn(move || {
            let filename = "./test.wav";
            let reader = hound::WavReader::open(filename).expect("Failed to open file");

            let samples: Vec<f64> = reader.into_samples::<i16>()
                .filter_map(Result::ok)
                .map(|s| s as f64)
                .collect();

            let mut current_window: Vec<f64> = vec![0.0; CHUNK_SIZE * 5];
            let start_time = Instant::now();

            for (i, chunk) in samples.chunks(CHUNK_SIZE).enumerate() {
                let downsampled_chunk: Vec<f64> = chunk.iter()
                    .step_by(DOWNSAMPLE_FACTOR)
                    .cloned()
                    .collect();

                let shift_amount = (downsampled_chunk.len() * 5).min(current_window.len());
                current_window.drain(..shift_amount);
                current_window.extend(downsampled_chunk.clone());

                {
                    let mut waveform_data = waveform_clone.lock().unwrap();
                    waveform_data.clear();
                    waveform_data.extend(&current_window);
                }

                {
                    let mut fft_data = fft_result_clone.lock().unwrap();
                    *fft_data = Self::compute_fft(&current_window);
                }

                let elapsed = start_time.elapsed().as_secs_f64();
                let expected_time = (i + 1) as f64 * time_per_frame;
                if elapsed < expected_time {
                    std::thread::sleep(Duration::from_secs_f64(expected_time - elapsed));
                }
            }

            *is_playing_clone.lock().unwrap() = false;
        });

        Self { waveform, fft_result, is_playing }
    }

    fn compute_fft(samples: &[f64]) -> Vec<f64> {
        let len = samples.len().next_power_of_two();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(len);

        let mut buffer: Vec<Complex<f64>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
        buffer.resize(len, Complex::new(0.0, 0.0));

        fft.process(&mut buffer);
        buffer.iter().take(len / 2).map(|c| c.norm()).collect()
    }
}
