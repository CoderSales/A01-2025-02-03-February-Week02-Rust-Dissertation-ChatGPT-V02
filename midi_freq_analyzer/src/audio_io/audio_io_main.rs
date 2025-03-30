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

    let mut last_print_time = std::time::Instant::now();

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

                let out_buf_len = data.len(); // ✅ capture length before mutable borrow
                let out_buf_len = data.len(); // ✅ move outside borrow scope
                for (i, sample) in data.iter_mut().enumerate() {
                    #[allow(unused)]
                    let input_amp = *input_gain_clone.lock().unwrap();

                    let raw_input = *buffer_copy.get(i + offset).unwrap_or(&0.0);
                    {
                        let mut ab = analysis_buffer_clone.lock().unwrap();
                        ab.push(raw_input);
                        if ab.len() >= 2048 {
                            static mut LAST_PRINT: Option<std::time::Instant> = None;
                            let now = std::time::Instant::now();
                            let should_print = unsafe {
                                match LAST_PRINT {
                                    Some(last) if now.duration_since(last).as_secs_f32() < 1.0 => false,
                                    _ => {
                                        LAST_PRINT = Some(now);
                                        true
                                    }
                                }
                            };

                            let (low, mid, high, _) = analyze_frequencies(&ab[..2048]);
                            let debug_line = "".to_string(); // 🔇 wipe residual 🎯 output
                            let debug_line = "".to_string(); // 🔇 kills multiline spam
                            let buffer_guard = buffer.lock().unwrap();
                            let input_peak = buffer_guard.iter().cloned().fold(0.0_f32, f32::max);
                            let max = input_peak;
                            let buffer_len = buffer_guard.len();
                            // let cli_line = build_cli_line(data.len(), output_peak, input_peak, max, buffer_len, data_len, bass_block, mid_block, high_block, &debug_line);
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
                                                                                                            
                            let cli_line = build_cli_line(
                                out_buf_len,
                                output_peak,
                                input_peak,
                                max,
                                buffer_len,
                                data_len,
                                bass_block,
                                mid_block,
                                high_block,
                                &debug_line,
                            );

                            let now = std::time::Instant::now();
                            if now.duration_since(last_print_time).as_secs_f32() >= 1.0 {
                                last_print_time = now;
                                print!("{}", cli_line);
                                use std::io::{stdout, Write};
                                stdout().flush().unwrap();
                            }

                            // if should_print {
                            print!("{}", cli_line);
                            use std::io::{stdout, Write};
                            stdout().flush().unwrap();
                            // }
                                                        

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

        fn build_cli_line(
            out_buf_len: usize,
            output_peak: f32,
            input_peak: f32,
            max: f32,
            buffer_len: usize,
            data_len: usize,
            bass: &str,
            mid: &str,
            high: &str,
            debug: &str,
        ) -> String {
            format!(
                "\r🔁 OutBuf: {:<4} | 🔊 Out: {:.6} | 🎙️ In: {:.6} | 🎚 Max: {:.6} | 🎧 Buffers: {} in / {} out | 🎵 B:{} M:{} H:{} | debug: {}",
                out_buf_len, output_peak, input_peak, max,
                buffer_len, data_len, bass, mid, high, debug
            )
        }

    println!("🔎 Requested output buffer size: {:?}", shared_config.buffer_size);
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
