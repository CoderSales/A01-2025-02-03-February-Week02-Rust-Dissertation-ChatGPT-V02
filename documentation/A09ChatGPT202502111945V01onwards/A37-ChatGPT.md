# ChatGPT

## Input: Bug E0583 and E0425

### cargo test Output (Bug)

error[E0583]: file not found for module `audio`
 --> src/main.rs:1:1
  |
1 | mod audio;
  | ^^^^^^^^^^
  |
  = help: to create the module `audio`, create file "src\audio.rs" or "src\audio\mod.rs"
  = note: if there is a `mod audio` elsewhere in the crate already, import it with `use crate::...` instead

error[E0425]: cannot find function `get_audio_device` in module `audio`
  --> src/main.rs:12:25
   |
12 |     let device = audio::get_audio_device();
   |                         ^^^^^^^^^^^^^^^^ not found in `audio`

error[E0425]: cannot find function `get_audio_config` in module `audio`
  --> src/main.rs:13:25
   |
13 |     let config = audio::get_audio_config(&device);
   |                         ^^^^^^^^^^^^^^^^ not found in `audio`

warning: unused import: `cpal::traits::StreamTrait`
 --> src/main.rs:4:5
  |
4 | use cpal::traits::StreamTrait;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

Some errors have detailed explanations: E0425, E0583.
For more information about an error, try `rustc --explain E0425`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer" test) generated 1 warning
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer" test) due to 3 previous errors; 1 warning emitted
warning: build failed, waiting for other jobs to finish...

### Priority

Make code compile

### Ignore

Warning
