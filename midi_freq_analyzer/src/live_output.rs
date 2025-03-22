use std::time::{Instant, Duration};
use std::sync::Mutex;
use std::io::Write; // ✅ Required for flush()


static LAST_UPDATE: Mutex<Option<Instant>> = Mutex::new(None);

/// Live amplitude visualization using `_` for simple horizontal bars
pub fn print_live_amplitude(amplitude: f32) {
    let now = Instant::now();
    let mut last_update = LAST_UPDATE.lock().unwrap();

    // Update only every 10ms
    if let Some(last) = *last_update {
        if now.duration_since(last) < Duration::from_millis(10) {
            return;
        }
    }
    *last_update = Some(now);

    let level = (amplitude * 50.0) as usize;
    let bar = "_".repeat(level);

    // Ensure full overwrite by printing spaces first
    print!("\r{:50}\r[{}] ", " ", bar); 
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
