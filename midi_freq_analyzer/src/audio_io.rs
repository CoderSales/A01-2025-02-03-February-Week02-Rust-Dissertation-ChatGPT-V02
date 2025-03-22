use std::sync::{Arc, Mutex}; // ensure this is at top
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::time::Duration;
use std::thread;
use crate::stream_setup;
use midi_freq_analyzer::audio;
use crate::bitrate;
use crate::constants::BUFFER_SIZE;
use crate::create_buffer;


pub fn start_audio_io(output_gain: Arc<Mutex<f32>>) {
    let input_gain = Arc::new(Mutex::new(1.0));
    let input_gain_clone = Arc::clone(&input_gain);

    let output_gain = Arc::new(Mutex::new(1.0)); // default gain = 1.0
    let output_gain_clone = Arc::clone(&output_gain);

    let _host = cpal::default_host();
    let host = cpal::default_host();
    let input_device = host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap().contains("Intel® Smart Sound"))
        .expect("Intel mic not found");
    let output_device = cpal::default_host().default_output_device().expect("No default output device");

    let output_config = audio::get_audio_config(&output_device);
    let output_size = BUFFER_SIZE;

    bitrate::print_audio_bitrate(&output_config);

    let buffer = create_buffer(output_size);

    println!("\nUsing input device: {}\n", input_device.name().unwrap());
    let buffer = create_buffer(output_size);
    let buffer_clone = Arc::clone(&buffer);
    let sample_rate = output_config.sample_rate.0;
    let stream = output_device
        .build_output_stream(
            &output_config.into(),
            move |data: &mut [f32], _| {
                let buffer = buffer_clone.lock().unwrap();
                let offset = buffer.len().saturating_sub(data.len());
                for (i, sample) in data.iter_mut().enumerate() {
                    let gain = *output_gain_clone.lock().unwrap();
                    *sample = buffer.get(i + offset).unwrap_or(&0.0) * gain;
                }

                let output_peak = data.iter().cloned().fold(0.0_f32, f32::max);
                let input_peak = buffer.iter().cloned().fold(0.0_f32, f32::max);
                let max = input_peak;

                let bar = |val: f32| if val > 0.002 { "|-|" } else { "|_|" };
                let bass_bar = bar(max * 0.8);
                let mid_bar = bar(max * 0.9);
                let high_bar = bar(max);

                use std::io::{stdout, Write};
                print!("\x1B[4F\x1B[0J");
                print!(
                    "🔊 Output peak: {:.6}\n🎧 Output buffer size: {}, Input buffer size: {}\n🎵 Bass: {} Mid: {} High: {} 🦚 Max amplitude: {:.6}\n🎩 Input peak: {:.6}\n",
                    output_peak,
                    data.len(),
                    buffer.len(),
                    bass_bar,
                    mid_bar,
                    high_bar,
                    max,
                    input_peak
                );
                stdout().flush().unwrap();
            },
            move |err| eprintln!("Stream error: {:?}", err),
            None,
        )
        .expect("❌ Failed to build output stream: Unsupported config");
    println!("Using output device: {}", output_device.name().unwrap());
    println!("\n\n\n\n");
    
    let input_config = audio::get_audio_config(&input_device);
    let data_clone_for_input = Arc::clone(&buffer);
    let input_stream = stream_setup::setup_audio_stream(
        &input_device,
        &input_config,
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
}
