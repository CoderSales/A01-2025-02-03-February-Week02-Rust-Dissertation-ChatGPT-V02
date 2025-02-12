impl eframe::App for Visualization {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Audio Visualization");

            // UI Buttons for Listening
            if ui.button("🎤 Listen").clicked() {
                self.is_listening = true;
            }
            if ui.button("🛑 Stop Listening").clicked() {
                self.is_listening = false;
            }

            // Lock the data (Immutable Borrow)
            let waveform_data = self.audio.waveform.lock().unwrap();
            let fft_data = self.audio.fft_result.lock().unwrap();
            let dominant_freq = *self.audio.dominant_frequency.lock().unwrap();

            // Display Plots
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

            // Drop Immutable Locks Before Calling Mutating Methods
            drop(waveform_data);
            drop(fft_data);

            // Display Detected Frequency & Chord
            ui.label(format!("Dominant Frequency: {:.2} Hz", dominant_freq));
            ui.label(format!("Chord: {}", Visualization::detect_chord(dominant_freq)));

            // ✅ Ensure buttons trigger `start_listening()` and `stop_listening()` safely
            if ui.button("🎤 Listen").clicked() {
                self.audio.start_listening();
                self.is_listening = true;
            }
            if ui.button("🛑 Stop Listening").clicked() {
                self.audio.stop_listening();
                self.is_listening = false;
            }
        });

        ctx.request_repaint();
    }
}
