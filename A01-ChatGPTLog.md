# ChatGPT

## Prompt

Please give me starting code to load an audio file into Rust.

## Response

Steps:
Add hound to your Cargo.toml dependencies:

toml
Copy
Edit
[dependencies]
hound = "3.5"
Create a Rust file (e.g., main.rs) and use the following code:

rust
Copy
Edit
use hound;

fn main() {
    let filename = "example.wav"; // Change this to your WAV file
    let reader = match hound::WavReader::open(filename) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            return;
        }
    };

    let spec = reader.spec();
    println!("Sample Rate: {}", spec.sample_rate);
    println!("Bits per Sample: {}", spec.bits_per_sample);
    println!("Channels: {}", spec.channels);

    let samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();
    println!("Loaded {} samples", samples.len());
}
Notes:
This example works for WAV files.
Modify the filename variable to match your file path.
It reads and prints basic audio properties.
It loads samples into a Vec<i16>, assuming 16-bit audio.
Would you like help with other formats (MP3, FLAC)? 🚀







