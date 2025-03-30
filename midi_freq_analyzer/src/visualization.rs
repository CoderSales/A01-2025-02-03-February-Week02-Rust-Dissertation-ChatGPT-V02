use crate::audio2::AudioProcessor2;
use eframe::egui::{self, CentralPanel, Button};
use egui_plot::{Plot, Line, PlotPoints};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex}; // ✅ Add this

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
        if self.is_listening {
            let _ = self.audio.lock().unwrap().start_listening();
        }        
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Audio Visualization");

            if ui.button("🎤 Listen").clicked() {
                println!("🎤 Listen button pressed!");
                self.audio.lock().unwrap().start_listening();
                self.is_listening = true;
            }
            if ui.button("🛑 Stop Listening").clicked() {
                self.audio.lock().unwrap().stop_listening();
                self.is_listening = false;
                self.audio.lock().unwrap().play_recorded_audio(); // ✅ Play recorded sound after stopping
            }

            if ui.button("🔄 Toggle Live/File").clicked() {
                self.is_file_mode = !self.is_file_mode;
            }

            if ui.button("📊 Analyse").clicked() {
                let dominant_freq: f64 = *self.audio.lock().unwrap().dominant_frequency.lock().unwrap();
                self.last_chord = Visualization::detect_chord(dominant_freq);
                ui.label(format!("🎵 Chord: {}", self.last_chord));                
            }

            let audio = self.audio.lock().unwrap();

            let raw_data = audio.waveform.lock().unwrap().clone();
            let max_amp = raw_data.iter().cloned().fold(0.0_f64, |a, b| a.max(b.abs())).max(1e-9);
            let normalized: Vec<f64> = raw_data.iter().map(|x| x / max_amp).collect();
            let mean = normalized.iter().copied().sum::<f64>() / normalized.len().max(1) as f64;
            let centered = normalized.iter().map(|x| x - mean).collect::<Vec<f64>>();
            let waveform_data: Vec<f64> = centered.iter().map(|x| x.clamp(-1.0, 1.0)).collect();
            
            let fft_data = audio.fft_result.lock().unwrap();
            let dominant_freq = *audio.dominant_frequency.lock().unwrap();
            
            Plot::new("Waveform").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    waveform_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                );
                plot_ui.line(Line::new(points).name("Waveform"));
            });

            Plot::new("FFT").show(ui, |plot_ui| {
                let points = PlotPoints::new(
                    fft_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                );
                plot_ui.line(Line::new(points).name("FFT"));
            });


            println!("🔍 First 5 samples: {:?}", &waveform_data[..5.min(waveform_data.len())]);

            ui.label(format!("Dominant Frequency: {:.2} Hz", dominant_freq));
            ui.label(format!("Detected Chord: {}", self.last_chord));
        });

        ctx.request_repaint();
    }
}