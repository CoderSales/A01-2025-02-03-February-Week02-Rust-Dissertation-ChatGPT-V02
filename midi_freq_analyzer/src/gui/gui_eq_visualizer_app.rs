use std::sync::{Arc, Mutex};
use eframe::{egui, App};

pub struct EqVisualizerApp {
    pub low_freq: Arc<Mutex<f32>>,
    pub mid_freq: Arc<Mutex<f32>>,
    pub high_freq: Arc<Mutex<f32>>,
}

impl App for EqVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚 EQ Visualizer");
            ui.separator();

            ui.horizontal_centered(|ui| {
                for (label, arc) in [
                    ("Low", &self.low_freq),
                    ("Mid", &self.mid_freq),
                    ("High", &self.high_freq),
                ] {
                    ui.allocate_ui_with_layout(
                        egui::vec2(40.0, 120.0),
                        egui::Layout::bottom_up(egui::Align::Center),
                        |ui| {
                            let val = *arc.lock().unwrap();
                            let filled_height = 100.0 * val.clamp(0.0, 1.0);

                            let (rect, _) =
                                ui.allocate_exact_size(egui::vec2(20.0, 100.0), egui::Sense::hover());
                            let painter = ui.painter_at(rect);

                            let filled_rect = egui::Rect::from_min_max(
                                egui::pos2(rect.left(), rect.bottom() - filled_height),
                                egui::pos2(rect.right(), rect.bottom()),
                            );

                            painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);
                            painter.rect_filled(filled_rect, 4.0, egui::Color32::LIGHT_BLUE);

                            ui.label(format!("{} {:.3}", label, val));
                        },
                    );
                }
            });
        });
    }
}