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


pub fn analyze_frequencies(samples: &[f32]) -> (f32, f32, f32) {
    let mut low = 0.0;
    let mut mid = 0.0;
    let mut high = 0.0;
    let mut spectrum = Vec::new();

    for (i, &sample) in samples.iter().enumerate() {
        let freq = (i as f32) * (44100.0 / samples.len() as f32);
        let magnitude = sample.abs();
        spectrum.push((freq, magnitude));

        if freq < 250.0 {
            low += magnitude;
        } else if freq < 4000.0 {
            mid += magnitude;
        } else {
            high += magnitude;
        }
    }

    spectrum.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🧪 Debug Spectrum Top 5:");
    for (freq, amp) in spectrum.iter().take(5) {
        println!(" - {:.1} Hz | Amp: {:.6}", freq, amp);
    }

    static mut FRAME_COUNT: usize = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 10 == 0 {
            let mut out = String::from("🎯 ");
            for (freq, amp) in spectrum.iter().take(3) {
                if *amp > 0.0001 && *freq < 20000.0 {
                    let note = frequency_to_note(*freq);
                    let cents = 1200.0 * (freq / note_to_freq(&note)).log2();
                    let sign = if cents >= 0.0 { "+" } else { "-" };
                    out += &format!("{} ({}{:>2.0}c) ", note, sign, cents.abs());
                }
            }
            if out.len() > 2 {
                print!("\r{}", out);
                io::stdout().flush().unwrap();
            }
        }
    }

    display_amplitude(low, mid, high);
    (low, mid, high)
}


fn note_to_freq(note: &str) -> f32 {
    match note {
        "A4" => 440.0,
        "E8" => 5274.0,
        "D#8" => 4978.0,
        _ => 440.0, // fallback
    }
}



pub fn display_amplitude(low: f32, mid: f32, high: f32) {
    let bass_block = if low > 0.1 { "|-|" } else { "|_|" };
    let mid_block = if mid > 0.1 { "|-|" } else { "|_|" };
    let high_block = if high > 0.1 { "|-|" } else { "|_|" };

    // print!("\r🎵 Bass: {} Mid: {} High: {} ", bass_block, mid_block, high_block);
    std::io::stdout().flush().unwrap();
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

            let (low, mid, high) = analyze_frequencies(&vec![0.0; 2048]); // Placeholder buffer

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
