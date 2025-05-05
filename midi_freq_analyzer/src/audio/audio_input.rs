// src/audio/audio_input.rs

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use num_traits::ToPrimitive;

use crate::buffer::AudioBuffer;

use std::thread;


pub fn start_input_stream(buffer: Arc<Mutex<AudioBuffer>>) -> cpal::Stream {
    let host = cpal::default_host();
    
    let device = detect_loudest_input_device(300)
    .or_else(|| host.default_input_device())
    .expect("No suitable input device found");

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

pub fn list_input_devices() {
    let host = cpal::default_host();
    if let Ok(devices) = host.input_devices() {
    for device in devices {
        println!("🎤 Found input device: {}", device.name().unwrap_or("Unnamed".into()));
    }
    } else {
       println!("⚠️ No input devices found.");
    }
}

pub fn detect_loudest_input_device(duration_ms: u64) -> Option<cpal::Device> {
    let host = cpal::default_host();
    let devices = host.input_devices().ok()?;

    let mut best_device = None;
    let mut highest_peak = 0.0;

    for device in devices {
        let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        let config = device.default_input_config().ok()?;
        let sample_format = config.sample_format();
        let config = config.into();

        let peak = match sample_format {
            cpal::SampleFormat::F32 => test_peak::<f32>(&device, &config, duration_ms),
            cpal::SampleFormat::I16 => test_peak::<i16>(&device, &config, duration_ms),
            cpal::SampleFormat::U16 => test_peak::<u16>(&device, &config, duration_ms),
            _ => continue,
        };

        if peak > highest_peak {
            highest_peak = peak;
            best_device = Some((device, name, peak));
        }
    }

    best_device.map(|(d, n, p)| {
        println!("🎧 Selected: {} (peak {:.5})", n, p);
        d
    })
}

fn test_peak<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    duration_ms: u64
) -> f32
where
    T: cpal::Sample + cpal::SizedSample + num_traits::ToPrimitive,
    {
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    let peak = Arc::new(Mutex::new(0.0f32));
    let peak_clone = peak.clone();

    let stream = match device.build_input_stream(
        config,
        move |data: &[T], _| {
            let max = data
                .iter()
                .filter_map(|s| s.to_f32())
                .map(f32::abs)
                .fold(0.0f32, f32::max);
            let mut p = peak_clone.lock().unwrap();
            *p = p.max(max);
        },
        |_| {},
        None,
    ) {
        Ok(s) => s,
        Err(_) => return 0.0,
    };

    if stream.play().is_err() {
        return 0.0;
    }

    std::thread::sleep(Duration::from_millis(duration_ms));
    drop(stream);

    let val = *peak.lock().unwrap();
    val
}

pub fn start_device_scanner_thread(shared: Arc<Mutex<Vec<(String, f32, cpal::Device)>>>) {
    thread::spawn(move || loop {
        let host = cpal::default_host();
        let mut new_list = vec![];

        if let Ok(devices) = host.input_devices() {
            for device in devices {
                let name = device.name().unwrap_or("Unnamed".into());
                if let Ok(config) = device.default_input_config() {
                    let peak = match config.sample_format() {
                        cpal::SampleFormat::F32 => test_peak::<f32>(&device, &config.into(), 300),
                        cpal::SampleFormat::I16 => test_peak::<i16>(&device, &config.into(), 300),
                        cpal::SampleFormat::U16 => test_peak::<u16>(&device, &config.into(), 300),
                        _ => continue,
                    };
                    new_list.push((name, peak, device));
                }
            }
        }

        if let Ok(mut list) = shared.lock() {
            *list = new_list;
        }

        std::thread::sleep(std::time::Duration::from_secs(4));
    });
}

pub fn switch_input_stream(
    device: cpal::Device,
    buffer: Arc<Mutex<AudioBuffer>>,
    stream_handle: Arc<Mutex<Option<cpal::Stream>>>,
) {
    // stop previous stream
    if let Some(old_stream) = stream_handle.lock().unwrap().take() {
        drop(old_stream);
    }

    let config = match device.default_input_config() {
        Ok(c) => c,
        Err(_) => return,
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), buffer),
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), buffer),
        cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), buffer),
        _ => return,
    };

    stream.play().ok();

    *stream_handle.lock().unwrap() = Some(stream);
}