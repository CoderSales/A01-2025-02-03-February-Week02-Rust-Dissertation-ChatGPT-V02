use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

use std::io::{self, Write}; // ✅ Add this at the top

pub fn select_audio_device(host: &cpal::Host, is_input: bool) -> cpal::Device {
    let devices = if is_input {
        host.input_devices().expect("Failed to get input devices")
    } else {
        host.output_devices().expect("Failed to get output devices")
    };

    let device_list: Vec<_> = devices.collect();
    println!(
        "\nAvailable {} devices:",
        if is_input { "input" } else { "output" }
    );

    let device_list = if is_input { host.input_devices() } else { host.output_devices() }
    .expect("Failed to get devices")
    .collect::<Vec<_>>();

    for (i, device) in device_list.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("Unknown".to_string()));
    }

    print!("Select {} device (Enter number): ", if is_input { "input" } else { "output" });
    std::io::stdout().flush().unwrap();
    
    loop {
        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection).expect("Failed to read input");
        if let Ok(index) = selection.trim().parse::<usize>() {
            if let Some(device) = device_list.get(index) {
                break device.clone();
            }
        }
        println!("Invalid selection, try again.");
    }
}
