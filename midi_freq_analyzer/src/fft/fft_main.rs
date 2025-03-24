use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[allow(unused)]
use cpal::traits::{DeviceTrait, HostTrait};
#[allow(unused)]
use mlua::{Lua, Result};
use std::io::{self, Write};
use crate::notes::frequency_to_note;
use std::collections::VecDeque;
// static mut NOTE_HISTORY: Option<VecDeque<String>> = None;
use std::collections::HashMap;
use crate::output_handler::print_cli_line;
use crate::output_handler::bind_gui_output;
pub static mut NOTE_HISTORY: Option<VecDeque<String>> = None;
pub static mut SPECTRUM_SCROLL: Option<VecDeque<String>> = None;
use crate::fft::fft_analyze_frequencies::analyze_frequencies;



pub fn note_to_freq(note: &str) -> f32 {
    let note_frequencies = [
        ("C", 261.63), ("C#", 277.18), ("D", 293.66), ("D#", 311.13),
        ("E", 329.63), ("F", 349.23), ("F#", 369.99), ("G", 392.00),
        ("G#", 415.30), ("A", 440.00), ("A#", 466.16), ("B", 493.88),
    ];

    // Try to extract note letter and octave from string like "A4"
    if note.len() < 2 || note.len() > 3 {
        return 440.0; // fallback
    }

    let (base, octave) = note.split_at(note.len() - 1);
    let octave: i32 = octave.parse().unwrap_or(4);
    let base_freq = note_frequencies
        .iter()
        .find(|(n, _)| *n == base)
        .map(|(_, f)| *f)
        .unwrap_or(440.0);

    base_freq * 2.0f32.powf((octave as f32 - 4.0))
}



pub fn display_amplitude(low: f32, mid: f32, high: f32) {
    let _bass_block = if low > 0.1 { "|-|" } else { "|_|" };
    let _mid_block = if mid > 0.1 { "|-|" } else { "|_|" };
    let _high_block = if high > 0.1 { "|-|" } else { "|_|" };

    // print!("\r🎵 Bass: {} Mid: {} High: {} ", bass_block, mid_block, high_block);
    // std::io::stdout().flush().unwrap();
}

#[allow(unused)]
#[derive(Default)]
pub struct AudioApp {
    status_message: String,
    log_output: Arc<Mutex<String>>,
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl App for AudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Audio Analyzer");
            ui.separator();

            if ui.button("▶ Record").clicked() {
                self.status_message = "Recording...".to_string();
                let log_output = Arc::clone(&self.log_output);
                thread::spawn(move || {
                    let mut log = log_output.lock().unwrap();
                    *log = String::new();
                    for i in 1..=10 {
                        thread::sleep(Duration::from_millis(500));
                        log.push_str(&format!("✅ Processing samples... {}\n", i));
                    }
                });
            }
            if ui.button("⏹ Stop").clicked() {
                self.status_message = "Stopped.".to_string();
            }
            ui.label(&self.status_message);
            ui.separator();

            let log = self.log_output.lock().unwrap();
            ui.add_sized([400.0, 200.0], egui::TextEdit::multiline(&mut log.clone()));
        });
    }
}

#[derive(Default)]
struct FrequencyMeter {
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl App for FrequencyMeter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚 Frequency Levels");

            let (low, mid, high, _) = analyze_frequencies(&vec![0.0; 2048]); // Placeholder buffer

            *self.low_freq.lock().unwrap() = low;
            *self.mid_freq.lock().unwrap() = mid;
            *self.high_freq.lock().unwrap() = high;

            ui.vertical(|ui| {
                ui.add(egui::ProgressBar::new(low / 20000.0).show_percentage());
                ui.label("Low Frequencies (20Hz - 250Hz)");
                ui.add(egui::ProgressBar::new(mid / 20000.0).show_percentage());
                ui.label("Mid Frequencies (250Hz - 4kHz)");
                ui.add(egui::ProgressBar::new(high / 20000.0).show_percentage());
                ui.label("High Frequencies (4kHz - 20kHz)");
            });
        });
    }
}

pub fn launch_gui() -> Result<()> {
    let options = NativeOptions::default();
    let low_freq = Arc::new(Mutex::new(0.0));
    let mid_freq = Arc::new(Mutex::new(0.0));
    let high_freq = Arc::new(Mutex::new(0.0));

    let gui_display = Arc::new(Mutex::new(String::new()));
    bind_gui_output(Arc::clone(&gui_display));

    let freq_meter = FrequencyMeter {
        low_freq,
        mid_freq,
        high_freq,
    };

    eframe::run_native(
        "Frequency Meter",
        options,
        Box::new(|_cc| Ok(Box::new(freq_meter))),
    )
    .unwrap();

    Ok(())
}
