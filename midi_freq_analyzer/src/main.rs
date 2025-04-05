// src/main2.rs

mod audio {
    pub mod audio_input;
}


mod buffer;
mod analytics {
    pub mod waveform_analytics;
    pub mod frequency_analytics;
}
mod gui {
    pub mod waveform_gui;
    pub mod gui2;
}
mod pipeline {
    pub mod waveform_pipeline;
    pub mod frequency_pipeline;
}
mod app {
    pub mod main_ui;
}

use app::main_ui::AudioApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Rust Audio Visualizer",
        options,
        Box::new(|_cc| Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(AudioApp::new())))
    )
}