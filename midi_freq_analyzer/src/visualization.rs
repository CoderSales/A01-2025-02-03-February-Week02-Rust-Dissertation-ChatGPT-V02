    use crate::audio2::AudioProcessor2;
    use eframe::egui::{self, CentralPanel, Button};
    // use egui_plot::{Plot, Line, PlotPoints};
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
            // println!("✅ Visualization::new() started");
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
            println!("✅ update() entered");
            if self.is_listening {
                let _ = self.audio.lock().unwrap().start_listening();
                self.is_listening = false;
            }        
            if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // ⬇️ your original update() content

                static mut LAST_FRAME: Option<Instant> = None;

                let now = Instant::now();
                let repaint_allowed = unsafe {
                    match LAST_FRAME {
                        Some(prev) => now.duration_since(prev).as_millis() > 33, // ~30 FPS
                        None => true,
                    }
                };


                
        
                // CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
                //     ui.label("🚧 GUI running");
                //     // ui.label("🚧 GUI running");

                //     // println!("🧱 CentralPanel visible");
                //     ui.heading("Live Audio Visualization");

                //     if ui.button("🎤 Listen").clicked() {
                //         // println!("🎤 Listen button pressed!");
                //         self.audio.lock().unwrap().start_listening();
                //         self.is_listening = true;
                //     }
                //     if ui.button("🛑 Stop Listening").clicked() {
                //         self.audio.lock().unwrap().stop_listening();
                //         self.is_listening = false;
                //         self.audio.lock().unwrap().play_recorded_audio(); // ✅ Play recorded sound after stopping
                //     }
                //     println!("✅ update() line 97");

                //     if ui.button("🔄 Toggle Live/File").clicked() {
                //         self.is_file_mode = !self.is_file_mode;
                //     }

                //     if ui.button("📊 Analyse").clicked() {
                //         let dominant_freq: f64 = *self.audio.lock().unwrap().dominant_frequency.lock().unwrap();
                //         self.last_chord = Visualization::detect_chord(dominant_freq);
                //         ui.label(format!("🎵 Chord: {}", self.last_chord));                
                //     }
                //     println!("✅ update() line 108");

                //     let audio = self.audio.lock().unwrap();

                //     let raw_data = audio.waveform.lock().unwrap().clone();
                //     // println!("🔍 raw_data.len() = {}", raw_data.len());
                //     let max_amp = raw_data.iter().cloned().fold(0.0_f64, |a, b| a.max(b.abs())).max(MIN_NORMALIZATION_AMPLITUDE);

                //     let gain = self.audio.lock().unwrap().gain;
                //     let normalized: Vec<f64> = raw_data.iter().map(|x| x * gain / max_amp).collect();

                //     let mean = normalized.iter().copied().sum::<f64>() / normalized.len().max(1) as f64;
                //     let centered = normalized.iter().map(|x| x - mean).collect::<Vec<f64>>();
                //     // let waveform_data: Vec<f64> = centered.iter().map(|x| x.clamp(-1.0, 1.0)).collect();
                    
                //     // let waveform_data: Vec<f64> = centered
                //     //     .iter()
                //     //     .map(|x| {
                //     //         // let amp = x.abs().clamp(1e-12, 1.0); // avoid log(0)
                //     //         // let amp = 1e-3; // dummy safe fallback
                //     //         let amp: f64 = 1e-3;
                //     //         20.0 * amp.log10()
                //     //     })
                //     //     .collect();

                //     // let waveform_data: Vec<f64> = vec![-20.0; 512];

                                            
                // plot.show(ui, |_plot_ui| {});

                //             ui.label("📉 Skipped plot rendering (testing)");


                //             ui.label("Time (ms)");
                //             ui.label("Amplitude (dB)"); // TODO: add dB scale
                //             ui.horizontal(|row| {
                //                 row.label("Time (ms)");
                //                 row.separator();
                //                 row.label("Amplitude (dB)");
                //             });

                //             // start of moved code inside of closure.

                //             let waveform_data: Vec<f64> = vec![-40.0; 512];


                //             // println!("📉 Waveform (5): {:?}", &waveform_data[..5.min(waveform_data.len())]);
        
                            
                //             let fft_data = self.audio.lock().unwrap().fft_result.lock().unwrap();
                //             let dominant_freq = *self.audio.lock().unwrap().dominant_frequency.lock().unwrap();
                            
                //             // // Waveform plot:
                //             // Plot::new("Waveform (dB)")
                //             //     .include_y(-80.0)
                //             //     .include_y(0.0)
                //             //     .show(ui, |plot_ui| {
                //             //     let points = PlotPoints::new(
                //             //         waveform_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                //             //     );
                //             //     plot_ui.line(Line::new(points).name("Waveform"));
                //             // });
        
                //             // let points = PlotPoints::new(
                //             //     waveform_data
                //             //         .iter()
                //             //         .enumerate()
                //             //         // .map(|(i, &y)| [i as f64, y])
                //             //         .map(|(i, &y)| [(i as f64 * 1000.0 / SAMPLE_RATE as f64), y])
                //             //         // .map(|(i, &y)| [i as f64 * 1000.0 / SAMPLE_RATE as f64, y as f64])
                //             //         // .map(|(i, &y)| [(i as f64 * 1000.0 / SAMPLE_RATE), y])
        
        
                //             //         .collect::<Vec<[f64; 2]>>(),
                //             // );
                //             let points = PlotPoints::new(
                //                 (0..512).map(|x| [x as f64, -40.0]).collect::<Vec<[f64; 2]>>()
                //             );
                            
        
        
                //             println!("✅ update() line 157");
        
                //             let plot = Plot::new("Waveform (dB)")
                //                 .include_y(-120.0)
                //                 .include_y(0.0)
                //                 .label_formatter(|name, value| {
                //                     format!("{name}: {:.1} ms, {:.1} dB", value.x, value.y)
                //                 });
        

                //             // end of moved code inside of closure.

                //             ui.allocate_ui_with_layout(
                //                 egui::vec2(ui.available_width(), 300.0),
                //                 egui::Layout::top_down(egui::Align::LEFT),
                //                 |ui| {
        
                //                     // plot.show(ui, |plot_ui| {
                //                     //     plot_ui.line(Line::new(points).name("Waveform"));
                //                     // });
        
                //         },
                //     );
                                        


                //     // Patch Example
                //     // let dt_avg = self.audio.lock().unwrap().dt_ms.load(Ordering::Relaxed); // or manually track avg
                //     // pub dt_ms: AtomicF64; // later add possibly.
                //     ui.label("Δt ≈ [not tracked]");
                //     ui.label("Δt data not available yet");
                //     println!("✅ update() almost exited");
                //     // ui.label(format!("Δt ≈ {:.2} ms", dt_avg));


                //     // Plot::new("FFT").show(ui, |plot_ui| {
                //     //     let points = PlotPoints::new(
                //     //         fft_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                //     //     );
                //     //     plot_ui.line(Line::new(points).name("FFT"));
                //     // });

                //     // let (peak_index, _) = fft_data
                //     //     .iter()
                //     //     .enumerate()
                //     //     .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                //     //     .unwrap_or((0, &0.0));

                //     // let peak_freq = peak_index as f64 * 44100.0 / 512.0; // SAMPLE_RATE / CHUNK_SIZE

                //     // Plot::new("Peak Frequency").show(ui, |plot_ui| {
                //     //     let points = PlotPoints::new(
                //     //         fft_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
                //     //     );
                //     //     plot_ui.line(Line::new(points).name("FFT"));
                    
                //     //     let (peak_index, _) = fft_data
                //     //         .iter()
                //     //         .enumerate()
                //     //         .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                //     //         .unwrap_or((0, &0.0));
                    
                //     //     let peak_freq = peak_index as f64 * 44100.0 / 512.0;
                    
                //     //     plot_ui.text(
                //     //         Text::new(
                //     //             PlotPoint::new(peak_index as f64, fft_data[peak_index]),
                //     //             format!("Peak: {:.2} Hz", peak_freq),
                //     //         )
                //     //         .color(Color32::YELLOW)
                //     //         .anchor(egui::Align2::CENTER_TOP),
                //     //     );
                //     // });

                //     // let freq = *self.audio.lock().unwrap().dominant_frequency.lock().unwrap();
                //     // self.last_chord = Visualization::detect_chord(freq);

                //     // ui.label(format!("🎯 Frequency: {:.2} Hz", freq));
                //     // ui.label(format!("🎸 Chord: {}", self.last_chord));


                //     // // println!("🔍 First 5 samples: {:?}", &waveform_data[..5.min(waveform_data.len())]);

                //     // ui.label(format!("Dominant Frequency: {:.2} Hz", dominant_freq));
                //     // ui.label(format!("Detected Chord: {}", self.last_chord));
                // });

                egui::Window::new("Audio App").show(ctx, |ui| {
                    ui.heading("🎧 Live Audio Visualization");
                
                    if ui.button("🎤 Listen").clicked() {
                        println!("🎤 Listen clicked");
                    }
                    if ui.button("🛑 Stop Listening").clicked() {
                        println!("🛑 Stop clicked");
                    }
                    if ui.button("🔄 Toggle Live/File").clicked() {
                        println!("🔄 Toggle clicked");
                    }
                    if ui.button("📊 Analyse").clicked() {
                        println!("📊 Analyse clicked");
                    }
                
                    let audio = self.audio.lock().unwrap();
                    let raw_data = audio.waveform.lock().unwrap().clone();
                    let max_amp = raw_data.iter().cloned().fold(0.0_f64, |a, b| a.max(b.abs())).max(1e-12_f64);
                    let normalized: Vec<f64> = raw_data.iter().map(|x| x / max_amp).collect();
                    let centered = normalized.iter().map(|x| x - normalized.iter().copied().sum::<f64>() / normalized.len().max(1) as f64).collect::<Vec<f64>>();
                    let waveform_data: Vec<f64> = centered.iter().map(|x| {
                        let amp = x.abs().clamp(1e-12, 1.0);
                        20.0 * amp.log10()
                    }).collect();
                    let points = PlotPoints::new(
                        waveform_data.iter().enumerate()
                            .map(|(i, &y)| [(i as f64 * 1000.0 / SAMPLE_RATE as f64), y])
                            .collect()
                    );
                
                    let plot = Plot::new("Waveform (dB)")
                        .include_y(-120.0)
                        .include_y(0.0)
                        .label_formatter(|name, value| {
                            format!("{name}: {:.1} ms, {:.1} dB", value.x, value.y)
                        });
                
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), 300.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            plot.show(ui, |plot_ui| {
                                plot_ui.line(Line::new(points).name("Waveform"));
                            });
                        },
                    );
                });
                                               

                // ctx.request_repaint();

                if repaint_allowed {
                    // ctx.request_repaint();
                    // unsafe {
                    //     LAST_FRAME = Some(now);
                    // }
                    if repaint_allowed {
                        ctx.request_repaint_after(Duration::from_millis(33)); // ~30 FPS
                    }                    
                }
                

            })) {
                // eprintln!("💥 GUI crashed: {:?}", e);
        }
    }

    // fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    //     if self.is_listening {
    //         println!("🎙️ Triggering start_listening()");
    //         let _ = self.audio.lock().unwrap().start_listening();
    //         self.is_listening = false; // prevent repeated calls
    //     }
        
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         let audio = self.audio.lock().unwrap();
    //         let raw_data = audio.waveform.lock().unwrap().clone();
    
    //         // let max_amp = raw_data.iter().cloned().fold(0.0_f64, |a, b| a.max(b.abs())).max(1e-9);
    //         // println!("🔍 Raw samples: {:?}", &raw_data[..5.min(raw_data.len())]);
    //         // let normalized: Vec<f64> = raw_data.iter().map(|x| x / max_amp).collect();
    //         // let mean = normalized.iter().copied().sum::<f64>() / normalized.len().max(1) as f64;
    //         // let centered = normalized.iter().map(|x| x - mean).collect::<Vec<f64>>();
    //         // let waveform_data: Vec<f64> = centered.iter().map(|x| x.clamp(-1.0, 1.0)).collect();

    //         let mean = raw_data.iter().copied().sum::<f64>() / raw_data.len().max(1) as f64;
    //         let centered: Vec<f64> = raw_data.iter().map(|x| x - mean).collect();
    //         let waveform_data: Vec<f64> = centered.iter().cloned().collect();


    //         let points = PlotPoints::new(
    //             waveform_data.iter().enumerate().map(|(i, &y)| [i as f64, y]).collect()
    //         );
    
    //         Plot::new("Waveform")
    //             .include_y(-1.0)
    //             .include_y(1.0)
    //             .show(ui, |plot_ui| {
    //                 plot_ui.line(Line::new(points).name("Waveform"));
    //         });
    //     });

    //     static mut LAST_GUI_TIME: Option<Instant> = None;

    //     let now = Instant::now();
    //     unsafe {
    //         if let Some(prev) = LAST_GUI_TIME {
    //             println!("⏱ GUI frame Δt = {:?}", now.duration_since(prev));
    //         }
    //         LAST_GUI_TIME = Some(now);
    //     }

    //     ctx.request_repaint(); // <-- force live repainting
    // }
            

}


    // impl Visualization {
    //     pub fn new() -> Self {
    //         println!("✅ Visualization::new() started");
    //         Self {
    //             audio: Arc::new(Mutex::new(AudioProcessor2::new())),
    //             is_listening: true,
    //             is_file_mode: false,
    //             last_analysis_time: Instant::now(),
    //             last_chord: "Unknown".to_string(),
    //         }
    //     }
    // }
