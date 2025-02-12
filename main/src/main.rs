mod audio;
mod visualization;

use audio::AudioVisualizer;
use eframe::NativeOptions;
use eframe::epaint::vec2;
use eframe::egui;
use rodio::Source;  // ✅ Fix: Import `Source` trait

fn main() {
    let filename = "./test.wav";
    let (_stream, stream_handle) = rodio::OutputStream::try_default().expect("Failed to create output stream");

    let file = std::fs::File::open(filename).expect("Failed to open file");
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).expect("Failed to decode audio");

    let reader = hound::WavReader::open(filename).expect("Failed to open file for duration check");
    let num_samples = reader.len() as f64;
    let sample_rate = reader.spec().sample_rate as f64;
    let audio_duration_secs = num_samples / sample_rate;

    let _ = stream_handle.play_raw(source.convert_samples());  // ✅ Fix works now

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(vec2(800.0, 500.0)), // ✅ Adjusted aspect ratio
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Real-Time Audio FFT Visualizer",
        options,
        Box::new(move |_cc| Box::new(AudioVisualizer::new(audio_duration_secs))),  
    ) {
        eprintln!("Error running eframe: {}", e);
    };
}
