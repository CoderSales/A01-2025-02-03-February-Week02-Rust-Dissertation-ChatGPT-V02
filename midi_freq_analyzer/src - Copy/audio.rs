use cpal::traits::{DeviceTrait, HostTrait}; // ✅ Fix import
use cpal::{StreamConfig, Device};
// use std::io;
// mod device_selection;  // ❌ Remove this line
// mod device_selection;  // ✅ Ensure module is declared at the top


// use std::io::{self, Write}; // ✅ Fix missing `flush()` method
#[allow(unused)]
use std::io::{self, Write}; // ✅ Keep only one correct import
#[allow(unused)]
use crate::device_selection::select_audio_device;


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
    device.default_input_config()
        .or_else(|_| device.default_output_config())
        .expect("❌ No supported input or output config found")
        .into()
}
