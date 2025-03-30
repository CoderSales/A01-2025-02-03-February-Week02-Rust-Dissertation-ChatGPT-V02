use std::sync::{Arc, Mutex}; // ensure this is at top
use crate::bitrate;
use crate::constants::BUFFER_SIZE;
use crate::mutex_handling::create_buffer;
use crate::stream_setup;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[allow(unused)]
use std::thread;
use std::time::Duration;
use crate::fft::fft_analyze_frequencies::analyze_frequencies;
use crate::output_handler::print_cli_line;


pub fn start_audio_io(output_gain: Arc<Mutex<f32>>, input_gain: Arc<Mutex<f32>>) {
    let input_gain_clone = Arc::clone(&input_gain);
    let output_gain_clone = Arc::clone(&output_gain);

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


    let analysis_buffer = Arc::new(Mutex::new(Vec::with_capacity(2048)));
    let analysis_buffer_clone = Arc::clone(&analysis_buffer);
    
    let buffer = create_buffer(BUFFER_SIZE);
    println!("\nUsing input device: {}\n", input_device.name().unwrap());

    let buffer_clone: Arc<Mutex<Vec<f32>>> = Arc::clone(&buffer); //  for output stream
    let buffer_clone_for_input: Arc<Mutex<Vec<f32>>> = Arc::clone(&buffer);    // for input stream
    let stream = output_device
        .build_output_stream(
            &shared_config.clone().into(),
            move |data: &mut [f32], _| {
                let buffer_guard = buffer_clone.lock().unwrap();
                let input_peak = buffer_guard.iter().cloned().fold(0.0, f32::max);
                let buffer_len = buffer_guard.len();
                let offset = buffer_len.saturating_sub(data.len());
                let buffer_copy: Vec<f32> = buffer_guard.clone(); // release borrow
                drop(buffer_guard); // unlock early ✅

                let output_peak = data.iter().cloned().fold(0.0_f32, f32::max);
                let data_len = data.len();

                for (i, sample) in data.iter_mut().enumerate() {
                    #[allow(unused)]
                    let input_amp = *input_gain_clone.lock().unwrap();

                    let raw_input = *buffer_copy.get(i + offset).unwrap_or(&0.0);
                    {
                        let mut ab = analysis_buffer_clone.lock().unwrap();
                        ab.push(raw_input);
                        if ab.len() >= 2048 {
                            let (low, mid, high, _) = analyze_frequencies(&ab[..2048]);
                            let debug_line = "".to_string(); // 🔇 wipe residual 🎯 output
                            let debug_line = "".to_string(); // 🔇 kills multiline spam
                            let buffer_guard = buffer.lock().unwrap();
                            let input_peak = buffer_guard.iter().cloned().fold(0.0_f32, f32::max);
                            let max = input_peak;
                            let buffer_len = buffer_guard.len();
                            drop(buffer_guard); // optional: release lock early

                            let bass_block: &str = match low {
                                x if x > 0.002 => "|-|",
                                x if x > 0.002 => "|-|",
                                _ => "|_|"
                            };
                            let mid_block = match mid {
                                x if x > 0.002 => "|-|",
                                x if x > 0.002 => "|-|",
                                _ => "|_|"
                            };
                            let high_block = match high {
                                x if x > 0.002 => "|-|",
                                x if x > 0.002 => "|-|",
                                _ => "|_|"
                            };
                                                                                                            
                            let cli_line: String = format!(
                                "🔊 Out: {:.6} | 🎙️ In: {:.6} | 🎚 Max: {:.6} | 🎧 Buffers: {} in / {} out | 🎵 B:{} M:{} H:{}  | debug: {}",
                                output_peak,
                                input_peak,
                                max,
                                buffer_len,
                                data_len,
                                bass_block,
                                mid_block,
                                high_block,
                                debug_line, // 🎯 Top notes from analyze_frequencies
                            );
                            
                        
                            print!("\r{}", cli_line);
                            use std::io::{stdout, Write};
                            stdout().flush().unwrap();
                                                    
                            ab.clear();
                        }
                    }

                    // if i == 0 {
                    //     println!("🎙️ Raw input sample[0]: {:.6}", raw_input);
                    // }
                    let gain = *output_gain_clone.lock().unwrap();
                    *sample = (raw_input * gain).clamp(-1.0, 1.0);
                }

            },
            move |err| eprintln!("Stream error: {:?}", err),
            None,
        )
        .expect("❌ Failed to build output stream: Unsupported config");

    println!("Using output device: {}", output_device.name().unwrap());
    std::thread::sleep(std::time::Duration::from_millis(500));
    // println!("\n\n\n\n");

    let data_clone_for_input = buffer_clone_for_input;
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
}
