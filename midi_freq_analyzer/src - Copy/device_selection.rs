use cpal::traits::{DeviceTrait, HostTrait}; // ✅ Fix import
use cpal::{StreamConfig, Device};
// use std::io;
// mod device_selection;  // ❌ Remove this line
// mod device_selection;  // ✅ Ensure module is declared at the top
// use crate::device_selection::select_audio_device; // ✅ Import correctly from main.rs
// ❌ causes circular import
// above proposed as redundant, but not present here in this file.
// use super::device_selection::select_audio_device;  // ✅ Use `super::` to access module


// use std::io::{self, Write}; // ✅ Fix missing `flush()` method
#[allow(unused)]
use std::io::{self, Write}; // ✅ Keep only one correct import


/// **User selects an input device at startup**
pub fn select_audio_device(is_input: bool) -> Device {
    let host = cpal::default_host();
    let devices: Vec<Device> = if is_input {
        host.input_devices().unwrap().collect()
    } else {
        host.output_devices().unwrap().collect()
    };

    println!("\nAvailable {} devices:", if is_input { "input" } else { "output" });
    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("Unknown".to_string()));
    }

    print!("Select {} device (Enter number): ", if is_input { "input" } else { "output" });
    use std::io::Write; // ✅ Import the `Write` trait ✅ Required for flush()
    std::io::stdout().flush().unwrap();

    let index = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<usize>() {
            if num < devices.len() {
                break num;
            }
        }
        println!("❌ Invalid selection. Please enter a valid number.");
    };

    devices.get(index).expect("Invalid selection").clone() // ✅ Corrected `devices`
}


/// **Gets default input device if user skips selection**
pub fn get_audio_device() -> Device {
    let host = cpal::default_host();

    println!("Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("- {}", device.name().unwrap_or("Unknown".to_string()));
    }

    host.input_devices()
        .unwrap()
        .find(|d| d.name().unwrap_or_default().contains("CABLE Output"))
        .or_else(|| host.default_input_device()) // ✅ Keep only one fallback
        .expect("No suitable audio input device found")
}

/// **Retrieves default audio config**
pub fn get_audio_config(device: &Device) -> StreamConfig {
    device.default_input_config().unwrap().into()
}
