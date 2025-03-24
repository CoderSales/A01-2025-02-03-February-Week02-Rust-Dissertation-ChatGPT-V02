use std::sync::{Arc, Mutex};

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
