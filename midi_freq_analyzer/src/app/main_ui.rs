// src/app/main_ui.rs

use eframe::egui::{self, CentralPanel, Ui};
use crate::buffer::AudioBuffer;
use crate::pipeline::waveform_pipeline::WaveformPipeline;
use crate::pipeline::frequency_pipeline::FrequencyPipeline;
use crate::audio::audio_input::start_input_stream;
use std::sync::{Arc, Mutex};

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

            let input_level = locked.samples.iter().map(|x| x.abs()).sum::<f32>() / locked.samples.len().max(1) as f32;
            ui.heading(format!("Audio Visualizer - Input Level: {:.2}", input_level));

            self.waveform.update(&locked);
            self.frequency.update(&locked);
            let waveform = self.waveform.update_return(&locked);
            self.waveform.gui().show_plot(ui, &waveform);
            self.frequency.show(ui, &locked);
            ctx.request_repaint();

            // TODO: Add GUI toggles, EQ sliders, visual thresholds, etc.
        });
    }
}

