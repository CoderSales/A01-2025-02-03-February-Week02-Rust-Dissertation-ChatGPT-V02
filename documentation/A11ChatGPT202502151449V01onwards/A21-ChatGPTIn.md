# Chat

## In

### CL Output

```bash
error[E0308]: mismatched types
   --> src\gui.rs:100:24
    |
100 |         Box::new(|_cc| Box::new(app)),
    |                        ^^^^^^^^^^^^^ expected `Result<Box<dyn App>, Box<dyn Error + Send + Sync>>`, found `Box<AudioApp>`
    |
    = note: expected enum `Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>`
             found struct `Box<AudioApp>`
help: try wrapping the expression in `Ok`
    |
100 |         Box::new(|_cc| Ok(Box::new(app))),
    |                        +++             +

error[E0308]: mismatched types
   --> src\gui.rs:107:24
    |
107 |         Box::new(|_cc| Box::new(freq_meter)),
    |                        ^^^^^^^^^^^^^^^^^^^^ expected `Result<Box<dyn App>, Box<dyn Error + Send + Sync>>`, found `Box<FrequencyMeter>`
    |
    = note: expected enum `Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>`
             found struct `Box<FrequencyMeter>`
help: try wrapping the expression in `Ok`
    |
107 |         Box::new(|_cc| Ok(Box::new(freq_meter))),
    |                        +++                    +

warning: unused variable: `note_clone`
  --> src/main.rs:51:9
   |
51 |     let note_clone = Arc::clone(&note_playing);
   |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_note_clone`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `last_note_clone`
  --> src/main.rs:52:9
   |
52 |     let last_note_clone = Arc::clone(&last_note);
   |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_last_note_clone`

warning: unused variable: `noise_profile`
  --> src/main.rs:54:9
   |
54 |     let noise_profile = if let Ok(profile) = load_noise_profile() {
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_noise_profile`

warning: unused variable: `peaks`
  --> src/main.rs:87:21
   |
87 |                 let peaks = fft::analyze_frequencies(&buffer[..buffer_len]);
   |                     ^^^^^ help: if this is intentional, prefix it with an underscore: `_peaks`

For more information about this error, try `rustc --explain E0308`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 4 warnings
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer") due to 2 previous errors; 4 warnings emitted
```
