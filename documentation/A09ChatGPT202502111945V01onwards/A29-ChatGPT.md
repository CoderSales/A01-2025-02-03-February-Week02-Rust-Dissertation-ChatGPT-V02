# ChatGPT

## Input

### Rust Output from cargo check

warning: unused imports: `DeviceTrait`, `HostTrait`, and `StreamTrait`
 --> src\audio.rs:5:20
  |
5 | use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
  |                    ^^^^^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `Button`
 --> src\visualization.rs:2:40
  |
2 | use eframe::egui::{self, CentralPanel, Button};
  |                                        ^^^^^^

warning: unused import: `Duration`
 --> src\visualization.rs:4:17
  |
4 | use std::time::{Duration, Instant};
  |                 ^^^^^^^^

error[E0599]: no method named `start_listening` found for struct `AudioProcessor` in the current scope
  --> src\visualization.rs:52:28
   |
52 |                 self.audio.start_listening();
   |                            ^^^^^^^^^^^^^^^ method not found in `AudioProcessor`
   |
  ::: src\audio.rs:7:1
   |
7  | pub struct AudioProcessor {
   | ------------------------- method `start_listening` not found for this struct

error[E0599]: no method named `stop_listening` found for struct `AudioProcessor` in the current scope
  --> src\visualization.rs:56:28
   |
56 |                 self.audio.stop_listening();
   |                            ^^^^^^^^^^^^^^ method not found in `AudioProcessor`
   |
  ::: src\audio.rs:7:1
   |
7  | pub struct AudioProcessor {
   | ------------------------- method `stop_listening` not found for this struct

error[E0599]: no method named `stop_listening` found for struct `AudioProcessor` in the current scope
  --> src\visualization.rs:92:28
   |
92 |                 self.audio.stop_listening();
   |                            ^^^^^^^^^^^^^^ method not found in `AudioProcessor`
   |
  ::: src\audio.rs:7:1
   |
7  | pub struct AudioProcessor {
   | ------------------------- method `stop_listening` not found for this struct

For more information about this error, try `rustc --explain E0599`.
warning: `main` (bin "main") generated 3 warnings
error: could not compile `main` (bin "main") due to 3 previous errors; 3 warnings emitted
