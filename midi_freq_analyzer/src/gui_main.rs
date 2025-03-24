use cpal::traits::{DeviceTrait, HostTrait};
use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use mlua::{Lua, Result}; // ✅ Keep Result

use crate::gui::gui_eq_visualizer_app::EqVisualizerApp;

use crate::gui::gui_io_gain_controls::GainControlApp;


// #[derive(Default)]
pub struct AudioApp {
    pub output_gain: Arc<Mutex<f32>>,
    pub input_gain: Arc<Mutex<f32>>,
    pub noise_samples: f64,
    pub noise_duration_ms: f64,
    status_message: String,
    log_output: Arc<Mutex<String>>,
    pub low_freq: Arc<Mutex<f32>>,
    pub mid_freq: Arc<Mutex<f32>>,
    pub high_freq: Arc<Mutex<f32>>,
}

impl Default for AudioApp {
    fn default() -> Self {
        Self {
            output_gain: Arc::new(Mutex::new(1.0)),
            input_gain: Arc::new(Mutex::new(1.0)), // ← Add this
            noise_duration_ms: 500.0,
            noise_samples: 10.0,            
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

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let mut gain = self.output_gain.lock().unwrap();
                    ui.add(egui::Slider::new(&mut *gain, 0.0..=5.0).vertical().text("🔊 Output"));
                    drop(gain);
                });
                ui.vertical(|ui| {
                    let mut input = self.input_gain.lock().unwrap();
                    ui.add(egui::Slider::new(&mut *input, 0.0..=10.0).vertical().text("🎙 Input"));
                    drop(input);
                });
            });            

            egui::CollapsingHeader::new("🎛️ Noise Control").default_open(true).show(ui, |ui| {
                if ui.button("🎧 Capture Noise").clicked() {
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
            });

            ui.horizontal(|ui| {
                ui.label("Samples:");
                ui.add(egui::DragValue::new(&mut self.noise_samples).range(1.0..=100.0));
                ui.label("ms/sample:");
                ui.add(egui::DragValue::new(&mut self.noise_duration_ms).range(100.0..=2000.0));
            });

            ui.add_space(8.0);
            ui.label(&self.status_message);
            ui.separator();

            let log = self.log_output.lock().unwrap();
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.add_sized([400.0, 200.0], egui::TextEdit::multiline(&mut log.clone()));
            });

            // Vertical EQ bars: Low, Mid, High
            ui.horizontal_centered(|ui| {
                for (label, arc) in [("Low", &self.low_freq), ("Mid", &self.mid_freq), ("High", &self.high_freq)] {
                    ui.allocate_ui_with_layout(egui::vec2(40.0, 120.0), egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        let val = *arc.lock().unwrap();
                        let filled_height = 100.0 * val.clamp(0.0, 1.0);

                        let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 100.0), egui::Sense::hover());
                        let painter = ui.painter_at(rect);

                        let filled_rect = egui::Rect::from_min_max(
                            egui::pos2(rect.left(), rect.bottom() - filled_height),
                            egui::pos2(rect.right(), rect.bottom()),
                        );

                        painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);
                        painter.rect_filled(filled_rect, 4.0, egui::Color32::LIGHT_BLUE);

                        ui.label(format!("{} {:.3}", label, val));
                    });
                }
            });
        });
    }
}

// Freq meter below


// #[derive(Default)]
// struct FrequencyMeter {
//     low_freq: Arc<Mutex<f32>>,
//     mid_freq: Arc<Mutex<f32>>,
//     high_freq: Arc<Mutex<f32>>,
// }

// impl App for FrequencyMeter {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("🎚 Frequency Levels");


// Freq meter above


//             // let low = *self.low_freq.lock().unwrap();
//             // let mid = *self.mid_freq.lock().unwrap();
//             // let high = *self.high_freq.lock().unwrap();

//             // ui.horizontal(|ui| {
//             //     ui.allocate_ui_with_layout(
//             //         egui::vec2(30.0, 120.0),
//             //         egui::Layout::bottom_up(egui::Align::Center),
//             //         |ui| {
//             //             let val = *self.low_freq.lock().unwrap();
//             //             let filled_height = 100.0 * val.clamp(0.0, 1.0);

//             //             let (rect, _) =
//             //                 ui.allocate_exact_size(egui::vec2(20.0, 100.0), egui::Sense::hover());
//             //             let painter = ui.painter_at(rect);

//             //             let filled_rect = egui::Rect::from_min_max(
//             //                 egui::pos2(rect.left(), rect.bottom() - filled_height),
//             //                 egui::pos2(rect.right(), rect.bottom()),
//             //             );

//             //             painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color); // background
//             //             painter.rect_filled(filled_rect, 4.0, egui::Color32::LIGHT_BLUE); // filled part

//             //             ui.label(format!("Low {:.3}", val));
//             //         },
//             //     );
//             //     ui.vertical(|ui| {
//             //         let v = *self.mid_freq.lock().unwrap();
//             //         ui.label(format!("Mid {:.3}", v));
//             //         ui.add(egui::ProgressBar::new(v).desired_height(100.0));
//             //     });
//             //     ui.vertical(|ui| {
//             //         let v = *self.high_freq.lock().unwrap();
//             //         ui.label(format!("High {:.3}", v));
//             //         ui.add(egui::ProgressBar::new(v).desired_height(100.0));
//             //     });
//             // });
//         });
//     }
// }


pub fn launch_gui(output_gain: Arc<Mutex<f32>>, input_gain: Arc<Mutex<f32>>) -> Result<()> {
    let low_freq = Arc::new(Mutex::new(0.5));
    let mid_freq = Arc::new(Mutex::new(0.7));
    let high_freq = Arc::new(Mutex::new(0.9));

    let audio_app = AudioApp {
        output_gain: Arc::clone(&output_gain),
        input_gain: Arc::clone(&input_gain),
        noise_samples: 10.0,
        noise_duration_ms: 500.0,
        status_message: "Idle".to_string(),
        log_output: Arc::new(Mutex::new(String::new())),
        low_freq: Arc::clone(&low_freq),
        mid_freq: Arc::clone(&mid_freq),
        high_freq: Arc::clone(&high_freq),
    };

    let eq_visualizer = EqVisualizerApp {
        low_freq: Arc::clone(&low_freq),
        mid_freq: Arc::clone(&mid_freq),
        high_freq: Arc::clone(&high_freq),
    };

    let gain_control_app = GainControlApp {
        output_gain: Arc::clone(&output_gain),
        input_gain: Arc::clone(&input_gain),
    };

    // Launch background windows
    std::thread::spawn(move || {
        eframe::run_native(
            "EQ Visualizer",
            Default::default(),
            Box::new(|_| Ok(Box::new(eq_visualizer))),
        );
    });

    std::thread::spawn(move || {
        eframe::run_native(
            "Gain Control",
            Default::default(),
            Box::new(|_| Ok(Box::new(gain_control_app))),
        );
    });

    // Primary GUI: Audio Analyzer (blocking)
    eframe::run_native(
        "Audio Analyzer",
        Default::default(),
        Box::new(|_| Ok(Box::new(audio_app))),
    ).unwrap();

    Ok(())
}
