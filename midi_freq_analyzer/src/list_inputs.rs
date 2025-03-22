use cpal::traits::{HostTrait, DeviceTrait};
use std::io::{self, Write};

pub fn print_input_devices() {
    let host = cpal::default_host();
    println!("🔍 Available input devices:");
    for device in host.input_devices().unwrap() {
        println!("🎙️ {}", device.name().unwrap());
    }

    println!("\n✅ Recommended:");
    println!("   Input:  Microphone Array (Intel® Smart Sound...)");
    println!("   Output: Headphones (Realtek(R) Audio) or similar physical device");

    print!("\n🔘 Press [Enter] to proceed...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}
