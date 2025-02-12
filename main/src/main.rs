mod audio;
mod visualization; // ✅ Ensure `visualization.rs` is properly included

use visualization::Visualization;
use eframe::NativeOptions;
use eframe::epaint::vec2;
use eframe::egui;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(vec2(800.0, 500.0)),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Real-Time Audio FFT & Chord Detection",
        options,
        Box::new(|_cc| Box::new(Visualization::new())),
    ) {
        eprintln!("Error running app: {}", e);
    };
}
