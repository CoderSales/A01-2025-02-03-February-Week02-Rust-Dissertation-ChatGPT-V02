use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn create_buffer(buffer_size: usize) -> Arc<Mutex<Vec<f32>>> {
    let buffer = Arc::new(Mutex::new(vec![0.0f32; buffer_size]));
    println!("🔍 Created Mutex buffer at {:p}", Arc::as_ptr(&buffer));
    buffer
}

pub fn create_panicked_threads() -> Arc<Mutex<HashSet<String>>> {
    let panicked_threads = Arc::new(Mutex::new(HashSet::new()));
    println!("🔍 Created Mutex panicked_threads at {:p}", Arc::as_ptr(&panicked_threads));
    panicked_threads
}

pub fn create_shared_data() -> Arc<Mutex<Vec<f32>>> {
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    println!("🔍 Created Mutex shared_data at {:p}", Arc::as_ptr(&shared_data));
    shared_data
}

pub fn create_note_playing() -> Arc<Mutex<bool>> {
    let note_playing = Arc::new(Mutex::new(false));
    println!("🔍 Created Mutex note_playing at {:p}", Arc::as_ptr(&note_playing));
    note_playing
}

pub fn create_last_note() -> Arc<Mutex<String>> {
    let last_note = Arc::new(Mutex::new("".to_string()));
    println!("🔍 Created Mutex last_note at {:p}", Arc::as_ptr(&last_note));
    last_note
}
