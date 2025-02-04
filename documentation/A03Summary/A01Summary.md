# Summary

## Commands

cargo new main

cd main/src

cargo build

cargo run

## Steps

### Load file

- use crate / dependency:

hound = "3.5"

### Playback file

- use crate / dependency:

rodio = "0.17"

#### Match

if Ok(r) then give r (the response)

if Error Err is the result then print Message, error and then return

### GUI

eframe = "0.24"
egui_plot = "0.24"

for the GUI

rustfft = "6"

for the FFT Calculation

#### rust Code in main.rs

##### struct

Add new struct AudioVisualizer

##### implementations

Add new implementation (impl) of AudioVisualizer struct

Add new implementation (impl) of eframe::App for AudioVisualizer

##### main method additional lines

```rust
let options = eframe::NativeOptions::default();
eframe::run_native(
    "Audio FFT Visualizer",
    options,
    Box::new(|_cc| Box::new(AudioVisualizer::new())),
);
```
