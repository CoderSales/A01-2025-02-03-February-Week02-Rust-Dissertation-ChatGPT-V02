// load audio with hound:
use hound;

// playback audio with rodio:
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // 1. load audio test.wav using hound crate:
    let filename = "test.wav"; // Change this to your WAV file
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
    
    
    // 2. playback audio test.wav using rodio crate:
    let filename = "test.wav"; // Change this to your file
    
    // Create an audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    
    // Open and decode the audio file
    let file = File::open(filename).expect("Failed to open file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
    
    // Play the audio
    stream_handle.play_raw(source.convert_samples()).expect("Failed to play audio");
    
    // Prevents premature termination (wait for playback to complete)
    std::thread::sleep(std::time::Duration::from_secs(5)); // Adjust based on file length
}

