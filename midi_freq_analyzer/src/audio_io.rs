use crate::bitrate;
use crate::constants::BUFFER_SIZE;
use crate::create_buffer;
use crate::stream_setup;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[allow(unused)]
use midi_freq_analyzer::audio;
use std::sync::{Arc, Mutex}; // ensure this is at top
use std::thread;
use std::time::Duration;

pub fn start_audio_io(output_gain: Arc<Mutex<f32>>, input_gain: Arc<Mutex<f32>>) {
    let input_gain_clone = Arc::clone(&input_gain);
    let output_gain_clone = Arc::clone(&output_gain);

    // let output_gain = Arc::new(Mutex::new(1.0));
    // let output_gain_clone = Arc::clone(&output_gain);

    let host = cpal::default_host();
    let input_device = host
        .input_devices()
        .unwrap()
        .find(|d| d.name().unwrap().contains("Intel® Smart Sound"))
        .expect("Intel mic not found");
    let output_device = host
        .default_output_device()
        .expect("No default output device");

    let mut shared_config = cpal::StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(48000),
        buffer_size: cpal::BufferSize::Fixed(BUFFER_SIZE as u32),
    };

    bitrate::print_audio_bitrate(&shared_config);

    let buffer = create_buffer(BUFFER_SIZE);
    println!("\nUsing input device: {}\n", input_device.name().unwrap());

    let buffer_clone = Arc::clone(&buffer);
    let stream = output_device
    .build_output_stream(
        &shared_config.clone().into(),
        move |data: &mut [f32], _| {
            let buffer = buffer_clone.lock().unwrap();
            let offset = buffer.len().saturating_sub(data.len());
            for (i, sample) in data.iter_mut().enumerate() {
                #[allow(unused)]
                let input_amp = *input_gain_clone.lock().unwrap();
                let raw_input = *buffer.get(i + offset).unwrap_or(&0.0);
                // if i == 0 {
                //     println!("🎙️ Raw input sample[0]: {:.6}", raw_input);
                // }
                let gain = *output_gain_clone.lock().unwrap();
                *sample = (raw_input * gain).clamp(-1.0, 1.0);
            }

            let output_peak = data.iter().cloned().fold(0.0_f32, f32::max);
            let input_peak = buffer.iter().cloned().fold(0.0_f32, f32::max);
            let max = input_peak;

            let bar = |val: f32| if val > 0.002 { "|-|" } else { "|_|" };
            let bass_bar = bar(max * 0.8);
            let mid_bar = bar(max * 0.9);
            let high_bar = bar(max);

            use std::io::{stdout, Write};
            print!("\r🔊 Output: {:.6} | 🎙️ Input: {:.6} | 🎚 Max: {:.6} | 🎧 Buffers: {} in / {} out       ",
                output_peak, input_peak, max, buffer.len(), data.len());
            stdout().flush().unwrap();
            },
        move |err| eprintln!("Stream error: {:?}", err),
        None,
    )
    .expect("❌ Failed to build output stream: Unsupported config");
    println!("Using output device: {}", output_device.name().unwrap());
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("\n\n\n\n");

    let data_clone_for_input = Arc::clone(&buffer);
    let input_stream = stream_setup::setup_audio_stream(
        &input_device,
        &mut shared_config,
        data_clone_for_input,
        Arc::clone(&input_gain),
    );

    stream.play().unwrap();
    input_stream.play().unwrap();

    let _output_stream = stream;
    let _input_stream = input_stream;

    loop {
        thread::sleep(Duration::from_secs(1));
    }

    // input_gain
}
