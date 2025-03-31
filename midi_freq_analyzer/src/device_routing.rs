// src/device_routing.rs
// pub mod device_routing;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

pub fn select_input_device(host: &cpal::Host) -> cpal::Device {
    // host.default_input_device().expect("No input device available")

    host.input_devices()
        .unwrap()
        .find(|d| {
            let name = d.name().unwrap_or_default();
            if name.contains("Headset") || name.contains("Earphone") {
                println!("🎧 Using headset mic: {}", name);
                true
            } else if name.contains("Microphone Array") {
                println!("🎙️ Using built-in mic: {}", name);
                true
            } else {
                false
            }
        })
        .expect("❌ No suitable input device found")
}

pub fn select_output_device(host: &Host) -> Device {
    host.output_devices()
        .unwrap()
        .find(|d| {
            let name = d.name().unwrap_or_default();
            if name.contains("Headphones") || name.contains("Earphone") {
                println!("🎧 Routing to earphones: {}", name);
                true
            } else if name.contains("Speakers") {
                println!("🔊 Routing to built-in speakers: {}", name);
                true
            } else {
                false
            }
        })
        .expect("❌ No suitable output device found")
}
