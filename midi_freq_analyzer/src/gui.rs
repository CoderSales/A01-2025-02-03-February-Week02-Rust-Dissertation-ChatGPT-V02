use eframe::egui; // Remove `epi`
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct AudioApp {
    status_message: String,
    log_output: Arc<Mutex<String>>, // Shared log output
    low_freq: Arc<Mutex<f32>>,      // Low frequency meter
    mid_freq: Arc<Mutex<f32>>,      // Mid frequency meter
    high_freq: Arc<Mutex<f32>>,     // High frequency meter
}

impl eframe::App for AudioApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Audio Analyzer");
            ui.separator();
            
            if ui.button("▶ Record").clicked() {
                self.status_message = "Recording...".to_string();
                let log_output = Arc::clone(&self.log_output);
                thread::spawn(move || {
                    for i in 1..=10 {
                        thread::sleep(Duration::from_millis(500));
                        let mut log = log_output.lock().unwrap();
                        log.push_str(&format!("✅ Processing samples... {}", i));
                    }
                });
            }

            if ui.button("⏹ Stop").clicked() {
                self.status_message = "Stopped.".to_string();
            }

            ui.label(&self.status_message);
            ui.separator();
            
            // Display real-time CLI output in the GUI
            let log = self.log_output.lock().unwrap();
            ui.add_sized([400.0, 200.0], egui::TextEdit::multiline(&mut log.clone()));
        });
    }
}

#[derive(Default, Clone)]
struct FrequencyMeter {
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl eframe::App for FrequencyMeter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚 Frequency Levels");
            
            let low = *self.low_freq.lock().unwrap();
            let mid = *self.mid_freq.lock().unwrap();
            let high = *self.high_freq.lock().unwrap();
            
            ui.add(egui::ProgressBar::new(low).show_percentage());
            ui.label("Low Frequencies");
            ui.add(egui::ProgressBar::new(mid).show_percentage());
            ui.label("Mid Frequencies");
            ui.add(egui::ProgressBar::new(high).show_percentage());
            ui.label("High Frequencies");
        });
    }
}

pub fn launch_gui() {
    let options = eframe::NativeOptions::default();

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

    // Run Audio Analyzer window
    eframe::run_native(
        "Audio Analyzer",
        options.clone(),  // Clone options to prevent move error
        Box::new(|_cc| Ok(Box::new(app))),  // ✅ Wrapped in Ok()
    )
    .unwrap();

    // Run Frequency Meter window
    eframe::run_native(
        "Frequency Meter",
        options,
        Box::new(|_cc| Ok(Box::new(freq_meter))),  // ✅ Wrapped in Ok()
    )
    .unwrap();
}
