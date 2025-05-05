// src/audio/audio_input.rs

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

use crate::buffer::AudioBuffer;

pub fn start_input_stream(buffer: Arc<Mutex<AudioBuffer>>) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), buffer),
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), buffer),
        cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), buffer),
        _ => panic!("Unsupported sample format"),
    };
        
    stream.play().expect("Failed to play input stream");
    stream
}

fn build_stream<T: cpal::Sample + cpal::SizedSample + num_traits::ToPrimitive>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    buffer: Arc<Mutex<AudioBuffer>>,
) -> cpal::Stream
where
    T: cpal::Sample,
{
    let channels = config.channels as usize;
    device
    .build_input_stream(
        config,
        move |data: &[T], _| {
            let mut buffer = buffer.lock().unwrap();
            buffer.samples.clear();
            buffer.samples.extend(data.iter().step_by(channels).map(|s| s.to_f32().unwrap_or(0.0)));
        },
        move |err| eprintln!("Stream error: {}", err),
        None, // fourth argument (latency hint)
    )
    .expect("Failed to build input stream")
}
