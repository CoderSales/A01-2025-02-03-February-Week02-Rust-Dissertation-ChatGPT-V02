use eframe::{egui, App};
use std::sync::{Arc, Mutex};

pub struct GainControlApp {
    pub output_gain: Arc<Mutex<f32>>,
    pub input_gain: Arc<Mutex<f32>>,
}

impl App for GainControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚 Gain Controls");
            ui.separator();

            ui.vertical(|ui| {
                let mut output = self.output_gain.lock().unwrap();
                ui.label("🔊 Output Gain");
                ui.add(egui::Slider::new(&mut *output, 0.0..=5.0).vertical());
            });

            ui.vertical(|ui| {
                let mut input = self.input_gain.lock().unwrap();
                ui.label("🎙 Input Gain");
                ui.add(egui::Slider::new(&mut *input, 0.0..=10.0).vertical());
            });
        });
    }
}