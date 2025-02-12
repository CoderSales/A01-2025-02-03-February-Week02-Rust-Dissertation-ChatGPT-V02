use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioProcessor {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub dominant_frequency: Arc<Mutex<f64>>,
    stream: Option<cpal::Stream>,
}

impl AudioProcessor {
    pub fn new() -> Self {
        Self {
            waveform: Arc::new(Mutex::new(vec![0.0; 256])),
            fft_result: Arc::new(Mutex::new(vec![0.0; 128])),
            dominant_frequency: Arc::new(Mutex::new(0.0)),
            stream: None,
        }
    }

    pub fn play_recorded_audio(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open("recorded_audio.wav").expect("Audio file not found"));
        let source = Decoder::new(file).unwrap();

        sink.append(source);
        sink.sleep_until_end(); // Block until audio finishes playing
    }
}
