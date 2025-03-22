use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cpal::traits::{DeviceTrait, HostTrait};

use mlua::{Lua, Result}; // ✅ Keep Result


// #[derive(Default)]
pub struct AudioApp {
    pub output_gain: Arc<Mutex<f32>>,
    pub input_gain: Arc<Mutex<f32>>, // ← Add this line
    status_message: String,
    log_output: Arc<Mutex<String>>,
    low_freq: Arc<Mutex<f32>>,
    mid_freq: Arc<Mutex<f32>>,
    high_freq: Arc<Mutex<f32>>,
}

impl Default for AudioApp {
    fn default() -> Self {
        Self {
            output_gain: Arc::new(Mutex::new(1.0)),
            input_gain: Arc::new(Mutex::new(1.0)), // ← Add this
            status_message: "Idle".to_string(),
            log_output: Arc::new(Mutex::new(String::new())),
            low_freq: Arc::new(Mutex::new(0.5)),
            mid_freq: Arc::new(Mutex::new(0.5)),
            high_freq: Arc::new(Mutex::new(0.5)),
        }
    }
}


impl App for AudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Audio Analyzer");
            ui.separator();

            let mut gain = self.output_gain.lock().unwrap();
            ui.add(egui::Slider::new(&mut *gain, 0.0..=5.0).text("🔊 Output Gain"));
            drop(gain); // release lock early


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

// #[derive(Default)]
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

pub fn launch_gui() -> Result<()> { // ✅ Change return type
    let options = NativeOptions::default();
    let host = cpal::default_host();
    let devices = host.devices().expect("Failed to get audio devices");
    let mut _selected_device = None;

    for device in devices {
        println!("Found device: {}", device.name().unwrap_or("Unknown".to_string()));
        if device.name().unwrap_or("Unknown".to_string()).contains("Microphone") {
            _selected_device = Some(device);
            break;
        }
    }

    // ✅ Define frequency variables before cloning
    let low_freq = Arc::new(Mutex::new(0.5)); // Default values
    let mid_freq = Arc::new(Mutex::new(0.7));
    let high_freq = Arc::new(Mutex::new(0.9));
    
    // let log_output = Arc::new(Mutex::new(String::new()));
    let low_freq_lua = Arc::clone(&low_freq);
    let mid_freq_lua = Arc::clone(&mid_freq);
    let high_freq_lua = Arc::clone(&high_freq);
    
    // ✅ Initialize Lua
    let lua = Lua::new();

    // ✅ Expose `set_eq` function to Lua
    let eq_table = lua.create_table()?;
    eq_table.set("set_eq", lua.create_function(move |_, (low, mid, high): (f32, f32, f32)| {
        *low_freq_lua.lock().unwrap() = low;
        *mid_freq_lua.lock().unwrap() = mid;
        *high_freq_lua.lock().unwrap() = high;
        println!("🎚 Lua Updated EQ - Low: {}, Mid: {}, High: {}", low, mid, high);
        Ok(())
    })?)?;
    
    lua.globals().set("eq", eq_table)?;

    // ✅ Example Lua script (Runs at startup)
    let low = *low_freq.lock().unwrap();
    let mid = *mid_freq.lock().unwrap();
    let high = *high_freq.lock().unwrap();
    
    println!("🔍 Initial Lua Frequency Values - Low: {}, Mid: {}, High: {}", low, mid, high);
    
    lua.load(format!("eq.set_eq({}, {}, {})", low, mid, high)).exec()?;
        
    let app = AudioApp {
        output_gain: Arc::new(Mutex::new(1.0)),
        input_gain: Arc::new(Mutex::new(1.0)),
        status_message: "Idle".to_string(),
        log_output: Arc::new(Mutex::new(String::new())),
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

    Ok(())
}

