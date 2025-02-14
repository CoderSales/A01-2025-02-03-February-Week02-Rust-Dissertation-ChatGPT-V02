# ChatGPT

## In

### main.rs [edit]

#### lines 54 to 83 inclusive

```rust
// Edited: Ensure display_amplitude() is called live inside input stream processing
let stream = device.build_input_stream(
    &config,
    move |data: &[f32], _: &_| {
        let mut buffer = data_clone.lock().unwrap();
        buffer.extend_from_slice(data);

        if buffer.len() >= 2048 {
            let peaks = fft::analyze_frequencies(&buffer[..2048]);

            let mut silence_count = 0; // New
            let mut total_frames = 0; // New
            
            let raw_amplitude = buffer.iter().map(|&x| x.abs()).sum::<f32>() / buffer.len() as f32;
            fft::display_amplitude(raw_amplitude, &mut silence_count, &mut total_frames); // New

            analyze_amplitude(&buffer[..2048]);  

            buffer.clear();
        }
    },
    err_fn,
    None,
).expect("Failed to create stream");
```

### Test

```bash
error[E0603]: function `display_amplitude` is private
  --> src/main.rs:68:22
   |
68 |                 fft::display_amplitude(raw_amplitude, &mut silence_count, &mut total_frames); // New
   |                      ^^^^^^^^^^^^^^^^^ private function
   |
note: the function `display_amplitude` is defined here
  --> C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer\src\fft.rs:81:1
   |
81 | fn display_amplitude(amplitude: f32, silence_count: &mut usize, total_frames: &mut usize) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused variable: `note_clone`
  --> src/main.rs:41:9
   |
41 |     let note_clone = Arc::clone(&note_playing);
   |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_note_clone`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `last_note_clone`
  --> src/main.rs:42:9
   |
42 |     let last_note_clone = Arc::clone(&last_note);
   |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_last_note_clone`

warning: unused variable: `noise_profile`
  --> src/main.rs:44:9
   |
44 |     let noise_profile = if let Ok(profile) = load_noise_profile() {
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_noise_profile`

warning: unused variable: `peaks`
  --> src/main.rs:62:21
   |
62 |                 let peaks = fft::analyze_frequencies(&buffer[..2048]);
   |                     ^^^^^ help: if this is intentional, prefix it with an underscore: `_peaks`

For more information about this error, try `rustc --explain E0603`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 4 warnings
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer") due to 1 previous error; 4 warnings emitted
```

