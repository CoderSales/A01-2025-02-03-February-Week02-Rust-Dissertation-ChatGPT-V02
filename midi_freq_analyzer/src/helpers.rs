use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn spawn_audio_thread(
    panicked_threads: &Arc<Mutex<std::collections::HashSet<String>>>,
    output_gain: &Arc<Mutex<f32>>,
    input_gain: &Arc<Mutex<f32>>,
) {
    let panicked_threads = Arc::clone(panicked_threads);
    let output_gain = Arc::clone(output_gain);
    let input_gain = Arc::clone(input_gain);
    std::thread::spawn(move || {
        let thread_name = "Audio Processing Thread".to_string();
        if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::audio_io::start_audio_io(output_gain, input_gain);
        })) {
            eprintln!("⚠️ Thread panicked: {}", thread_name);
            let mut list = panicked_threads.lock().unwrap();
            list.insert(thread_name);
        }
    });
}


pub fn spawn_logger_thread(program_start: Instant) {
    std::thread::spawn(move || {
        loop {
            let elapsed = program_start.elapsed().as_secs();
            if elapsed % 5 == 0 {
                println!("⏳ Program Running: {} seconds elapsed.", elapsed);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

