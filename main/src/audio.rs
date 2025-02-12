use std::sync::{Arc, Mutex};
use rustfft::{FftPlanner, num_complex::Complex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const CHUNK_SIZE: usize = 256;
const SAMPLE_RATE: f64 = 44100.0;  // ✅ Changed from `f32` to `f64`

pub struct AudioProcessor {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub dominant_frequency: Arc<Mutex<f64>>,
}

impl AudioProcessor {
    pub fn new() -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE / 2]));
        let dominant_frequency = Arc::new(Mutex::new(0.0));

        let waveform_clone = Arc::clone(&waveform);
        let fft_result_clone = Arc::clone(&fft_result);
        let dominant_frequency_clone = Arc::clone(&dominant_frequency);

        std::thread::spawn(move || {
            let host = cpal::default_host();
            let device = host.default_input_device().expect("No input device found");
            let config = device.default_input_config().unwrap().into();
            let stream = device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut waveform_data = waveform_clone.lock().unwrap();
                    waveform_data.clear();
                    waveform_data.extend(data.iter().map(|&s| s as f64));

                    let mut fft_data = fft_result_clone.lock().unwrap();
                    *fft_data = AudioProcessor::compute_fft(&waveform_data);

                    let mut dominant_freq = dominant_frequency_clone.lock().unwrap();
                    *dominant_freq = AudioProcessor::find_dominant_frequency(&fft_data);
                },
                |err| eprintln!("Stream error: {:?}", err),
            ).unwrap();
            stream.play().unwrap();
            loop { std::thread::sleep(std::time::Duration::from_millis(100)); }
        });

        Self { waveform, fft_result, dominant_frequency }
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

    fn find_dominant_frequency(fft_data: &[f64]) -> f64 {
        let max_index = fft_data.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        
        (max_index as f64) * (SAMPLE_RATE as f64 / CHUNK_SIZE as f64) // ✅ Convert SAMPLE_RATE to f64
    }    
}
