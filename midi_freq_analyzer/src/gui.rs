use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cpal::traits::{DeviceTrait, HostTrait};

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

            let low = *self.low_freq.lock().unwrap();
            let mid = *self.mid_freq.lock().unwrap();
            let high = *self.high_freq.lock().unwrap();

            ui.vertical(|ui| {
                ui.add(egui::ProgressBar::new(low).show_percentage());
                ui.label("Low Frequencies (20Hz - 250Hz)");
                ui.add(egui::ProgressBar::new(mid).show_percentage());
                ui.label("Mid Frequencies (250Hz - 4kHz)");
                ui.add(egui::ProgressBar::new(high).show_percentage());
                ui.label("High Frequencies (4kHz - 20kHz)");
            });
        });
    }
}

pub fn launch_gui() {
    let options = NativeOptions::default();
    let host = cpal::default_host();
    let devices = host.devices().expect("Failed to get audio devices");
    let mut selected_device = None;

    for device in devices {
        println!("Found device: {}", device.name().unwrap_or("Unknown".to_string()));
        if device.name().unwrap_or("Unknown".to_string()).contains("Microphone") {
            selected_device = Some(device);
            break;
        }
    }

    let log_output = Arc::new(Mutex::new(String::new()));
    let low_freq = Arc::new(Mutex::new(0.0));
    let mid_freq = Arc::new(Mutex::new(0.0));
    let high_freq = Arc::new(Mutex::new(0.0));

    let app = AudioApp {
        status_message: "Idle".to_string(),
        log_output: Arc::clone(&log_output),
        low_freq: Arc::clone(&low_freq),
        mid_freq: Arc::clone(&mid_freq),
        high_freq: Arc::clone(&high_freq),
    };

    let freq_meter = FrequencyMeter {
        low_freq,
        mid_freq,
        high_freq,
    };

    eframe::run_native(
        "Audio Analyzer",
        options.clone(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();

    eframe::run_native(
        "Frequency Meter",
        options,
        Box::new(|_cc| Ok(Box::new(freq_meter))),
    )
    .unwrap();
}
