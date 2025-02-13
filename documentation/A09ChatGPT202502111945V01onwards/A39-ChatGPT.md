# ChatGPT

## Input

### Commit message (used as a high level summary)

Edit audio rs Add line Fix use one dot in filename

### Summary of Fix for last issue

Issue was that file was saved as audio..rs not audio.rs

### Current Issue

cargo test
warning: constant `FFT_SIZE` is never used
 --> src\fft.rs:6:7
  |
6 | const FFT_SIZE: usize = 2048; // Larger FFT window
  |       ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `midi_freq_analyzer` (lib) generated 1 warning
warning: `midi_freq_analyzer` (lib test) generated 1 warning (1 duplicate)
   Compiling midi_freq_analyzer v0.1.0 (C:\Users\steph\OneDrive\Documents\48-Rust\A06ChatGPT\A01-proj\A03Project01\A01-proj\A01-2025-02-03-February-Week02-Rust-Dissertation-ChatGPT\midi_freq_analyzer)
error[E0599]: no method named `name` found for struct `Device` in the current scope
   --> src/main.rs:14:51
    |
14  |     println!("\nUsing input device: {}\n", device.name().unwrap());
    |                                                   ^^^^ method not found in `Device`
    |
   ::: C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cpal-0.15.3\src\traits.rs:102:8
    |
102 |     fn name(&self) -> Result<String, DeviceNameError>;
    |        ---- the method is available for `Device` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DeviceTrait` which provides `name` is implemented but not in scope; perhaps you want to import it
    |
1   + use cpal::traits::DeviceTrait;
    |

error[E0599]: no method named `build_input_stream` found for struct `Device` in the current scope
   --> src/main.rs:24:25
    |
24  |     let stream = device.build_input_stream(
    |                  -------^^^^^^^^^^^^^^^^^^
    |
   ::: C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cpal-0.15.3\src\traits.rs:125:8
    |
125 |     fn build_input_stream<T, D, E>(
    |        ------------------ the method is available for `Device` here
    |
    = help: items from traits can only be used if the trait is in scope
help: there is a method `build_input_stream_raw` with a similar name, but with different arguments
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cpal-0.15.3\src\traits.rs:181:5
    |
181 | /     fn build_input_stream_raw<D, E>(
182 | |         &self,
183 | |         config: &StreamConfig,
184 | |         sample_format: SampleFormat,
...   |
190 | |         D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
191 | |         E: FnMut(StreamError) + Send + 'static;
    | |_______________________________________________^
help: trait `DeviceTrait` which provides `build_input_stream` is implemented but not in scope; perhaps you want to import it
    |
1   + use cpal::traits::DeviceTrait;
    |

warning: unused import: `cpal::traits::StreamTrait`
 --> src/main.rs:3:5
  |
3 | use cpal::traits::StreamTrait;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0599`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer" test) generated 1 warning
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer" test) due to 2 previous errors; 1 warning emitted
