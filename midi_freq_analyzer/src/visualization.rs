    use crate::audio2::AudioProcessor2;
    use eframe::egui::{self, CentralPanel, Button};
    use egui_plot::{Plot, Line, PlotPoints, PlotPoint, Text};
    use std::time::{Duration, Instant};
    use std::sync::{Arc, Mutex};
    use egui::{Color32, Align2};
    use crate::constants::MIN_NORMALIZATION_AMPLITUDE;
    use crate::config::SAMPLE_RATE;


    pub struct Visualization {
        audio: Arc<Mutex<AudioProcessor2>>,
        is_listening: bool,
        is_file_mode: bool,  // ✅ Toggle between live input & file
        last_analysis_time: Instant,
        last_chord: String,
    }

    impl Visualization {
        pub fn new() -> Self {
            Self {
                audio: Arc::new(Mutex::new(AudioProcessor2::new())),
                is_listening: true,
                is_file_mode: false,  // ✅ Default to live input
                last_analysis_time: Instant::now(),
                last_chord: "Unknown".to_string(),
            }
        }

        fn detect_chord(frequency: f64) -> String {
            let note_frequencies = [
                ("C", 261.63), ("C#", 277.18), ("D", 293.66), ("D#", 311.13),
                ("E", 329.63), ("F", 349.23), ("F#", 369.99), ("G", 392.00),
                ("G#", 415.30), ("A", 440.00), ("A#", 466.16), ("B", 493.88),
            ];
            
            let mut closest_note = "Unknown";
            let mut min_diff = f64::MAX;

            for (note, freq) in note_frequencies.iter() {
                let diff = (freq - frequency).abs();
                if diff < min_diff {
                    min_diff = diff;
                    closest_note = note;
                }
            }
            format!("Detected: {}", closest_note)
        }
    }

    impl eframe::App for Visualization {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("✅ GUI Running");
                ui.label("This proves GUI is functional.");
            });
            ctx.request_repaint();
        }    }

