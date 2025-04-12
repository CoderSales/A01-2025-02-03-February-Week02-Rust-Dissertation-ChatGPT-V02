// src/app/main_ui.rs

use eframe::egui::{self, CentralPanel, Ui};
use crate::buffer::AudioBuffer;
use crate::pipeline::waveform_pipeline::WaveformPipeline;
use crate::pipeline::frequency_pipeline::FrequencyPipeline;
use crate::audio::audio_input::start_input_stream;
use std::sync::{Arc, Mutex};
use crate::analytics::note_label::frequency_to_note;
use crate::cli_log::log_status;


pub struct AudioApp {
    waveform: WaveformPipeline,
    frequency: FrequencyPipeline,
    buffer: Arc<Mutex<AudioBuffer>>,
    _stream: cpal::Stream,
}

impl AudioApp {
    pub fn new() -> Self {
        let buffer = Arc::new(Mutex::new(AudioBuffer::default()));
        let stream = start_input_stream(buffer.clone());
        Self {
            waveform: WaveformPipeline::new(),
            frequency: FrequencyPipeline::new(),
            buffer,
            _stream: stream,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            let Ok(locked) = self.buffer.lock() else {
                ui.label("Failed to lock buffer");
                return;
            };

            let input_level: f32 = locked.samples.iter().map(|x| x.abs()).sum::<f32>() / locked.samples.len().max(1) as f32;
            let amplitude = input_level;
            ui.heading(format!("Audio Visualizer - Input Level: {:.2}", input_level));

            self.waveform.update(&locked);
            self.frequency.update(&locked);
            let waveform = self.waveform.update_return(&locked);
            let y: f32 = self.waveform.y_range();
            let waveform_pipeline = &mut self.waveform;
            let (freq, len) = waveform_pipeline.dominant_frequency(&locked);
            let bin_width = 48_000.0 / len as f32;
            let bin_est = (freq / bin_width).round();
            let note_text: String = frequency_to_note(freq); // for GUI
            let note_text_fmt = if amplitude > 0.001 { // for CLI
                format!("{:<14}", frequency_to_note(freq))
            } else {
                format!("{:<14}", "---")
            };
            
            log_status(&format!(
                "smoothed_y: {:>7.4} | Note: {} | freq: {:>10.1} Hz | bin est: {:>4} | bin_w: {:>13.8}",
                y, note_text_fmt, freq, bin_est, bin_width
            ));
                                                                                    
            waveform_pipeline
                .gui()
                .show_plot(ui, &waveform, &locked, y, &note_text);
                
            
            self.frequency.show(ui, &locked);
            ctx.request_repaint();

            // TODO: Add GUI toggles, EQ sliders, visual thresholds, etc.
        });
    }
}

