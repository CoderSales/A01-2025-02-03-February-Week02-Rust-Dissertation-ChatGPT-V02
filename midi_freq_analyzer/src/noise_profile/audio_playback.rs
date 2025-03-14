use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// **Handles audio playback for a given buffer**
pub fn play_audio(buffer: Arc<Mutex<Vec<f32>>>) {
    loop {
        // 🔹 Step 1: Capture 10ms of input
        if let Ok(buffer) = buffer.lock() {
            let sample_size = buffer.len().min(10); // Prevent out-of-bounds
            println!("🎤 Capturing audio input... Sample: {:?}", &buffer[..sample_size]);
        }
        thread::sleep(Duration::from_millis(10));

        // 🔹 Step 2: Pause briefly
        thread::sleep(Duration::from_millis(10));

        // 🔹 Step 3: Play back output for 10ms
        println!("🔊 Playing back processed audio...");
        thread::sleep(Duration::from_millis(10));
    }
}
