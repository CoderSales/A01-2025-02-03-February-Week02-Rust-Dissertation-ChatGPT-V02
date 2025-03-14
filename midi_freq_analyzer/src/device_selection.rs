use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

use std::io::{self, Write}; // ✅ Add this at the top

pub fn select_audio_device(host: &Host, is_input: bool) -> Device {
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

    for (i, device) in device_list.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("Unknown".to_string()));
    }

    print!("Select {} device (Enter number): ", if is_input { "input" } else { "output" });
    std::io::stdout().flush().unwrap();
    
    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).expect("Failed to read input");
    let index: usize = selection.trim().parse().expect("Invalid input. Enter a number.");
    
    device_list.get(index).expect("Invalid selection").clone()
}
