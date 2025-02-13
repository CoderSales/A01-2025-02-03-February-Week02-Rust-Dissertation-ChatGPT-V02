use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{StreamConfig, Device};

use std::io;
use cpal::{Device, HostTrait};

pub fn select_audio_device() -> Device {
    let host = cpal::default_host();
    let devices: Vec<_> = host.input_devices().unwrap().collect();

    println!("\nAvailable input devices:");
    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("Unknown".to_string()));
    }

    print!("Select an input device (Enter number): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap_or(0);

    devices.get(index).expect("Invalid selection, using default.").clone()
}


/// Selects the best available input device
pub fn get_audio_device() -> Device {
    let host = cpal::default_host();

    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    // Select VB-Audio Virtual Cable if available, otherwise default input device
    host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device())
        .expect("No suitable audio input device found")
}

/// Retrieves default audio config
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}
