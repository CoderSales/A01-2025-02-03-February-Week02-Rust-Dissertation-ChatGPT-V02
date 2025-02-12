use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use std::sync::{Arc, Mutex};

fn main() {
    let host = cpal::default_host();
    let device = host
        .input_devices()
        .expect("Failed to get input devices")
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .expect("VB-Audio Virtual Cable not found");

    println!("Using input device: {}", device.name().unwrap());

    let config = device.default_input_config().unwrap();
    let sample_format = config.sample_format();
    let stream_config: StreamConfig = config.into();

    let data = Arc::new(Mutex::new(Vec::new()));

    let err_fn = |err| eprintln!("Error: {:?}", err);

    let data_clone = Arc::clone(&data);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &_| {
                let mut buffer = data_clone.lock().unwrap();
                buffer.extend_from_slice(data);
                println!("Captured {} samples", data.len());
            },
            err_fn,
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .expect("Failed to create stream");

    stream.play().expect("Failed to start stream");

    println!("Listening for audio... Press Ctrl+C to stop.");
    std::thread::sleep(std::time::Duration::from_secs(10));
}
