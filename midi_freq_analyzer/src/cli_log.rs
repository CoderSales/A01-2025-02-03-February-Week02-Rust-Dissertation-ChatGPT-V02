use std::io::{stdout, Write};

pub fn log_status(msg: &str) {
    const WIDTH: usize = 240;
    let trimmed = if msg.len() > WIDTH {
        &msg[..WIDTH]
    } else {
        msg
    };
    let padded = format!("{:<width$}", trimmed, width = WIDTH);
    print!("\r{}", padded);
    stdout().flush().unwrap();
}
