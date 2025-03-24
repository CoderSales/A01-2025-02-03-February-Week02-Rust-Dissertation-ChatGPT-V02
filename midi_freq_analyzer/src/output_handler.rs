// src/output_handler.rs

use std::io::{self, Write};

pub fn print_cli_line(line: &str) {
    print!("\r{}", line);
    io::stdout().flush().unwrap();
}
