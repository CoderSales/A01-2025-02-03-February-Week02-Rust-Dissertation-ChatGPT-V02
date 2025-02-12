use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    let devices = host.input_devices().expect("Failed to get input devices");

    println!("Available input devices:");
    for device in devices {
        println!("{}", device.name().unwrap_or("Unknown device".to_string()));
    }
}
