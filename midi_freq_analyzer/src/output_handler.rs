use std::io::{self, Write};
use std::sync::{Arc, Mutex};

static mut GUI_OUTPUT: Option<Arc<Mutex<String>>> = None;

/// Print one-liner in CLI (overwrites current line)
pub fn print_cli_line(line: &str) {
    print!("\r{}", line);
    io::stdout().flush().unwrap();

    // Also update shared GUI buffer if set
    unsafe {
        if let Some(shared) = &GUI_OUTPUT {
            let mut gui_line = shared.lock().unwrap();
            *gui_line = line.to_string();
        }
    }
}

/// (Optional) GUI can subscribe to updates
pub fn bind_gui_output(shared: Arc<Mutex<String>>) {
    unsafe {
        GUI_OUTPUT = Some(shared);
    }
}
