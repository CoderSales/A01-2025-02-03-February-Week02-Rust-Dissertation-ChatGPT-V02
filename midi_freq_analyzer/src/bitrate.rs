use cpal::StreamConfig;

/// Calculate and print audio bitrate information
pub fn print_audio_bitrate(config: &StreamConfig) {
    let sample_format = "f32"; // Assuming f32 format from CPAL
    let channels = config.channels;
    let sample_rate = config.sample_rate.0;

    let bit_depth = 32; // Default assumption for f32 format
    let bitrate = sample_rate as u32 * bit_depth * channels as u32;

    println!("\n🎵 Audio Configuration:");
    println!(" - Sample Format: {}", sample_format);
    println!(" - Channels: {}", channels);
    println!(" - Sample Rate: {} Hz", sample_rate);
    println!(" - Bit Depth: {} bits", bit_depth);
    println!(" - Calculated Bitrate: {} bps ({} kbps)", bitrate, bitrate / 1000);
}
