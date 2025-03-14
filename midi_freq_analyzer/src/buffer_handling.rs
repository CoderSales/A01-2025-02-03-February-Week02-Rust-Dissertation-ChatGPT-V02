use std::sync::{Arc, Mutex};

pub fn handle_buffer_lock<T>(buffer_clone: &Arc<Mutex<T>>, action: impl FnOnce(&mut T)) {
    match buffer_clone.lock() {
        Ok(mut buffer) => {
            action(&mut buffer);
        }
        Err(poisoned) => {
            println!(
                "🔍 Mutex poisoned in buffer handling! Mutex address: {:p}",
                Arc::as_ptr(buffer_clone)
            );
        }
    }
}
