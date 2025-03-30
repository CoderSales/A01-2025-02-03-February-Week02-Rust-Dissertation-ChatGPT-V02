use std::sync::{Arc, Mutex};
use rustfft::{FftPlanner, num_complex::Complex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::{BufReader, Write};
use rodio::{Decoder, OutputStream, Sink};

const CHUNK_SIZE: usize = 256;
const SAMPLE_RATE: f64 = 44100.0;

pub struct AudioProcessor2 {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub dominant_frequency: Arc<Mutex<f64>>,
    stream: Option<cpal::Stream>,
    recorded_audio: Arc<Mutex<Vec<f32>>>, // ✅ Store recorded audio
}

impl AudioProcessor2 {
    pub fn new() -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE / 2]));
        let dominant_frequency = Arc::new(Mutex::new(0.0));
        let recorded_audio = Arc::new(Mutex::new(Vec::new()));

        Self {
            waveform,
            fft_result,
            dominant_frequency,
            stream: None,
            recorded_audio,
        }
    }

    pub fn start_listening(&mut self) {
        let waveform_clone = Arc::clone(&self.waveform);
        let fft_result_clone = Arc::clone(&self.fft_result);
        let dominant_frequency_clone = Arc::clone(&self.dominant_frequency);
        let recorded_audio_clone = Arc::clone(&self.recorded_audio);

        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device found");
        let config = device.default_input_config().unwrap().into();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut waveform_data = waveform_clone.lock().unwrap();
                let mut recorded_audio = recorded_audio_clone.lock().unwrap();

                waveform_data.clear();
                for chunk in data.chunks(2) {
                    if let [left, _right] = chunk {
                        waveform_data.push(*left as f64);
                    }
                }

                recorded_audio.extend(data.iter());

                let mut fft_data = fft_result_clone.lock().unwrap();
                *fft_data = AudioProcessor2::compute_fft(&waveform_data);

                let mut dominant_freq = dominant_frequency_clone.lock().unwrap();
                *dominant_freq = AudioProcessor2::find_dominant_frequency(&fft_data);
                println!("🎧 Got {} samples. First: {:?}", data.len(), &data[..5.min(data.len())]);
            },
            |err| eprintln!("Stream error: {:?}", err),
            None,
        ).unwrap();

        println!("🚀 Before stream.play()");
        stream.play().unwrap();
        println!("✅ After stream.play()");

        self.stream = Some(stream);
    }

    pub fn stop_listening(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    pub fn play_recorded_audio(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let recorded_audio = self.recorded_audio.lock().unwrap();

        let path = "recorded_audio.wav";
        let mut writer = hound::WavWriter::create(path, hound::WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        }).unwrap();

        for &sample in recorded_audio.iter() {
            writer.write_sample((sample * i16::MAX as f32) as i16).unwrap();
        }
        writer.finalize().unwrap();

        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();
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
        
        (max_index as f64) * (SAMPLE_RATE / CHUNK_SIZE as f64)
    }
}