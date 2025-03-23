use std::sync::{Arc, Mutex};
use cpal::traits::DeviceTrait;
use crate::constants::BUFFER_SIZE;
use midi_freq_analyzer::fft::analyze_frequencies;

use crate::audio_io;


pub fn setup_audio_stream(
    device: &cpal::Device,
    config: &mut cpal::StreamConfig,
    data_clone: Arc<Mutex<Vec<f32>>>,
    input_gain: Arc<Mutex<f32>>,
) -> cpal::Stream {
    println!("🎛 Input config: {:?}", config);
    config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE as u32);
    device.build_input_stream(
        &config,
        {
            let data_clone = Arc::clone(&data_clone);
            move |data: &[f32], _: &_| {
                let mut buffer = data_clone.lock().unwrap();

                let buffer_len = buffer.len();
                let data_len = data.len();

                println!("🎧 Output buffer size: {}, Input buffer size: {}", data_len, buffer_len);

                let (_low, _mid, _high) = analyze_frequencies(data);
                let max = data.iter().cloned().fold(0.0_f32, f32::max);
                println!("🎚 Max amplitude: {:.6}", max);

                if buffer_len + data_len > BUFFER_SIZE {
                    buffer.drain(..buffer_len + data_len - BUFFER_SIZE);
                }
                println!("🎙️ Raw input sample[0]: {:.6}", data.get(0).unwrap_or(&0.0));
                buffer.extend_from_slice(data);

                let input_peak = data
                    .chunks(2)
                    .map(|frame| (frame.get(0).unwrap_or(&0.0) + frame.get(1).unwrap_or(&0.0)) / 2.0)
                    .fold(0.0_f32, f32::max);
                println!("🎙️ Input peak: {:.6}", input_peak);
                let left = data.get(0).unwrap_or(&0.0);
                let right = data.get(1).unwrap_or(&0.0);
                println!("🎙️ Raw L: {:.6} R: {:.6}", left, right);
            }
        },
        move |err| eprintln!("Stream error: {:?}", err),
        None,
    )
    .expect("Failed to create stream")
}
