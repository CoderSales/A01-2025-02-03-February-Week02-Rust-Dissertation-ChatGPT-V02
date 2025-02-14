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
