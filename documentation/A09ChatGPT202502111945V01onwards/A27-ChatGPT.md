# ChatGPT

## Input

____

### Next steps

Perform audio routing 

from Google Chrome Browser 

into program

so that 

chords can be played on 

[midi.city](https://midi.city/)

and the program

will analyse these

live.

### Output from cargo check

```bash
warning: unused import: `Write`
 --> src\audio.rs:5:26
  |
5 | use std::io::{BufReader, Write};
  |                          ^^^^^
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

warning: field `last_analysis_time` is never read
  --> src\visualization.rs:10:5
   |
6  | pub struct Visualization {
   |            ------------- field in this struct
...
10 |     last_analysis_time: Instant,
   |     ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `main` (bin "main") generated 4 warnings (run `cargo fix --bin "main"` to apply 3 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.85s
```

### Output from cargo build

```bash
warning: unused import: `Write`
 --> src\audio.rs:5:26
  |
5 | use std::io::{BufReader, Write};
  |                          ^^^^^
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

warning: field `last_analysis_time` is never read
  --> src\visualization.rs:10:5
   |
6  | pub struct Visualization {
   |            ------------- field in this struct
...
10 |     last_analysis_time: Instant,
   |     ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `main` (bin "main") generated 4 warnings (run `cargo fix --bin "main"` to apply 3 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.95s
```

### Notes from cargo run

Output to terminal was

```bash
Detected Chord: Detected: B
Detected Chord: Detected: B
Detected Chord: Detected: B
```

Slight delay between button presses (interface freezes for a few seconds).

However, this may be to be expected, and seems reasonable if the functionality works.

### Positive Note

file produced:

recorded_audio.wav

### Summary

Prioritise taking in input from [midi.city/](https://midi.city/).

____
