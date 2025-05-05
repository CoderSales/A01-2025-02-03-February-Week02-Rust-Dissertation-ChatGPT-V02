// src/main.rs

mod cli_log;

mod audio {
    pub mod audio_input;
}


mod buffer;
mod analytics {
    pub mod waveform_analytics;
    pub mod frequency_analytics;
    pub mod note_label;
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
    pub mod app_runner;
}

use app::main_ui::AudioApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    println!("🚀 Launching GUI");
    eframe::run_native(
        "Rust Audio Visualizer",
        options,
        Box::new(|_cc| Ok::<Box<dyn eframe::App>, _>(Box::new(AudioApp::new())))
    )
}