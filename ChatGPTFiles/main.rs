use hound;

fn main() {
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
}
