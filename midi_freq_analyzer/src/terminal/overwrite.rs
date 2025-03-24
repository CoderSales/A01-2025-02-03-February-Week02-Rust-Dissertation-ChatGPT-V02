use std::{thread::sleep, time::Duration};

fn main() {
    for i in 0..10 {
        print!("\rProcessing: {}%", i * 10);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
    }
    println!("\nDone!");
}
