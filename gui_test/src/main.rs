use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Test GUI", native_options, Box::new(|_cc| {
        Ok(Box::new(BasicApp))
    })).unwrap();
}

struct BasicApp;

impl eframe::App for BasicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("🧪 Hello GUI");
        });
    }
}
