// src/app/main_ui.rs

use eframe::egui::{self, CentralPanel, Ui};
use crate::buffer::AudioBuffer;
use crate::pipeline::waveform_pipeline::WaveformPipeline;
use crate::pipeline::frequency_pipeline::FrequencyPipeline;
use crate::audio::audio_input::start_input_stream;
use std::sync::{Arc, Mutex};
use crate::analytics::note_label::frequency_to_note;
use crate::cli_log::log_status;
use crate::audio::audio_input::start_device_scanner_thread;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::io::{stdout, Write};
static FIRST_UI: OnceLock<std::time::Instant> = OnceLock::new();
use std::sync::OnceLock;
static mut LAST_SWITCH: Option<String> = None;
static mut LAST_UI_PRINT: Option<std::time::Instant> = None;





pub struct AudioApp {
    waveform: WaveformPipeline,
    frequency: FrequencyPipeline,
    buffer: Arc<Mutex<AudioBuffer>>,
    stream_handle: Arc<Mutex<Option<cpal::Stream>>>,
    live_candidates: Arc<Mutex<Vec<(String, f32, cpal::Device)>>>,
}

impl AudioApp {
    pub fn new() -> Self {
        crate::audio::audio_input::list_input_devices();
        let buffer = Arc::new(Mutex::new(AudioBuffer::default()));
        let buffer_clone = buffer.clone();

        let stream = Self::dummy_stream();

        let live_candidates = Arc::new(Mutex::new(vec![]));
        start_device_scanner_thread(live_candidates.clone());


        Self {
            waveform: WaveformPipeline::new(),
            frequency: FrequencyPipeline::new(),
            buffer,
            stream_handle: Arc::new(Mutex::new(None)),
            live_candidates,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        static mut LAST_UI_PRINT: Option<std::time::Instant> = None;
        let now = std::time::Instant::now();
        let first = FIRST_UI.get_or_init(|| now);
        let elapsed = now.duration_since(*first);
        let secs = elapsed.as_secs();
        let millis = elapsed.subsec_millis();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        
        let since = format!("⏱ {:02}:{:02}:{:02}.{:03} since ui()", h, m, s, millis);
        let last = unsafe { LAST_SWITCH.clone().unwrap_or_default() };
        
        let status = format!("{}   {}", since, last);
        let padded = format!("{:<240}", status);
        
        unsafe {
            if LAST_UI_PRINT.map_or(true, |last| now.duration_since(last).as_secs_f32() > 1.0) {
                // overwrite old line fully
                print!("\r{: <240}\r", ""); // clear with 240 spaces first
                stdout().flush().unwrap();
            
                print!("{}", padded); // then print new
                stdout().flush().unwrap();
            
                LAST_UI_PRINT = Some(now);
            }
        }
        
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎹 MIDI Frequency Analyzer");
            ui.label("✅ GUI working without sweeps or audio input.");
            if let Ok(buffer) = self.buffer.lock() {
                ui.label(format!("🧪 Samples: {}", buffer.samples.len()));
                let peak = buffer.samples.iter().copied().map(f32::abs).fold(0.0, f32::max);
                ui.label(format!("📈 Peak amplitude: {:.5}", peak));
                let freq = self.waveform.dominant_frequency(&buffer).0;
                ui.label(format!("🎯 Frequency: {:.2} Hz", freq));
                ctx.request_repaint();
                ui.separator();
                ui.heading("🧪 GUI is alive!");
                if let Ok(candidates) = self.live_candidates.lock() {
                    ui.separator();
                    ui.heading("🎛 Input Devices:");

                    for (name, peak, device) in candidates.iter() {
                        ui.horizontal(|row| {
                            row.label(format!("• {} — peak: {:.5}", name, peak));
                            if row.button("Switch").clicked() {
                                let elapsed = now.duration_since(*FIRST_UI.get_or_init(|| now));
                                let secs = elapsed.as_secs();
                                let millis = elapsed.subsec_millis();
                                let h = secs / 3600;
                                let m = (secs % 3600) / 60;
                                let s = secs % 60;
                            
                                let msg = format!(
                                    "🔀 Switching to: {}  ⏱ {:02}:{:02}:{:02}.{:03}",
                                    name, h, m, s, millis
                                );
                            
                                let padded = format!("{:<240}", msg);
                                log_status(&padded);
                                let device_clone = device.clone();
                                
                                crate::audio::audio_input::switch_input_stream(
                                    device_clone,
                                    self.buffer.clone(),
                                    self.stream_handle.clone()
                                );

                                unsafe {
                                    LAST_SWITCH = Some(format!("🔀 {}", name));
                                }
                            }
                        });
                    }
                    
                }
            }
        });
    }

    fn dummy_stream() -> cpal::Stream {
        let host = cpal::default_host();
        host.default_input_device()
            .unwrap()
            .build_input_stream_raw(
                &cpal::StreamConfig {
                    channels: 1,
                    sample_rate: cpal::SampleRate(44100),
                    buffer_size: cpal::BufferSize::Default,
                },
                cpal::SampleFormat::F32,
                |_data, _| {},
                |_err| {},
                None,
            )
            .unwrap()
    }
}

