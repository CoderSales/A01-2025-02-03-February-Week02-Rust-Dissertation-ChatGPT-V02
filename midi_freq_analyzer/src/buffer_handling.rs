use std::sync::{Arc, Mutex};

pub fn handle_buffer_lock<T>(buffer_clone: &Arc<Mutex<T>>, action: impl FnOnce(&mut T)) {
    match buffer_clone.lock() {
        Ok(mut buffer) => {
            action(&mut buffer);
        }
        Err(poisoned) => {
            eprintln!("⚠️ Mutex poisoned in buffer handling: {:?}", poisoned);
        }
    }
}
