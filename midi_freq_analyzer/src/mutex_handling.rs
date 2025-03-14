use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn create_buffer(buffer_size: usize) -> Arc<Mutex<Vec<f32>>> {
    Arc::new(Mutex::new(vec![0.0f32; buffer_size]))
}

pub fn create_panicked_threads() -> Arc<Mutex<HashSet<String>>> {
    Arc::new(Mutex::new(HashSet::new()))
}

pub fn create_shared_data() -> Arc<Mutex<Vec<f32>>> {
    Arc::new(Mutex::new(Vec::new()))
}

pub fn create_note_playing() -> Arc<Mutex<bool>> {
    Arc::new(Mutex::new(false))
}

pub fn create_last_note() -> Arc<Mutex<String>> {
    Arc::new(Mutex::new("".to_string()))
}
