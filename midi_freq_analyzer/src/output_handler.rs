use std::io::{self, Write};
use std::sync::{Arc, Mutex};

static mut GUI_OUTPUT: Option<Arc<Mutex<String>>> = None;

/// Print one-liner in CLI (overwrites current line)
pub fn print_cli_line(line: &str) {
    const CLEAR_LEN: usize = 160;
    let padded = format!("{:<width$}", line, width = CLEAR_LEN);
    print!("\r{}", padded);
    io::stdout().flush().unwrap();
        io::stdout().flush().unwrap();

    // Append to log file
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("cli_log.txt")
    {
        let _ = writeln!(file, "{}", line);
    }
}

/// (Optional) GUI can subscribe to updates
pub fn bind_gui_output(shared: Arc<Mutex<String>>) {
    unsafe {
        GUI_OUTPUT = Some(shared);
    }
}
