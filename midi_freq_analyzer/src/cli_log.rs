use std::io::{stdout, Write};

pub fn log_status(msg: &str) {
    print!("\r{}", msg);
    stdout().flush().unwrap();
}
