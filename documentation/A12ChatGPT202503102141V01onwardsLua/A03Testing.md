# Testing

## Content


```bash
  |        ^^^^^^^^^^^

warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 36 warnings (run `cargo fix --bin "midi_freq_analyzer"` to apply 9 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.21s
     Running `C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer\target\debug\midi_freq_analyzer.exe`
🔍 Created Mutex panicked_threads at 0x1a8da005ce0
Found device: 40UHD_LCD_TV (HD Audio Driver for Display Audio)
Found device: Headphones (Realtek(R) Audio)
Found device: CABLE Input (VB-Audio Virtual Cable)
Found device: FxSound Speakers (FxSound Audio Enhancer)
Found device: Speakers (Realtek(R) Audio)
Found device: Microphone Array (Intel® Smart Sound Technology for Digital Microphones)
🔍 Initial Lua Frequency Values - Low: 0.5, Mid: 0.7, High: 0.9
🎚 Lua Updated EQ - Low: 0.5, Mid: 0.7, High: 0.9
🔍 Created Mutex buffer at 0x1a8da042640
⏳ Program Running: 0 seconds elapsed.

Available input devices:
0: Microphone Array (Intel® Smart Sound Technology for Digital Microphones)
1: Microphone (Realtek(R) Audio)
2: CABLE Output (VB-Audio Virtual Cable)
Select an input device (Enter number): 0

🎵 Audio Configuration:
 - Sample Format: f32
 - Channels: 2
 - Sample Rate: 48000 Hz
 - Bit Depth: 32 bits
 - Calculated Bitrate: 3072000 bps (3072 kbps)

Using input device: Microphone Array (Intel® Smart Sound Technology for Digital Microphones)

🔍 Created Mutex shared_data at 0x1a8e314cc50
🔍 Created Mutex note_playing at 0x1a8e1830e80
🔍 Created Mutex last_note at 0x1a8e314cf90
Capturing noise profile...
🔊 Capturing noise profile... Press Ctrl+C to stop.
[] ⏳ Program Running: 5 seconds elapsed.
[___] ⏳ Program Running: 10 seconds elapsed.
```

________

```bash
[] 🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
[] 🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
[] 🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
[__] 🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
[] 🔊 Playing back processed audio...
🎤 Capturing audio input... Sample: [0.0, 2e-10, 0.0, -3e-10, 1e-10, 1e-10, -1e-10, 3e-10, -1e-10, -3e-10]
```
