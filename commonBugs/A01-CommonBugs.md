# Common Bugs

## [E0433]

error[E0433]: failed to resolve: could not find `viewport` in `eframe`
   --> src/main.rs:159:27
    |
159 |         viewport: eframe::viewport::ViewportBuilder::default()
    |                           ^^^^^^^^ could not find `viewport` in `eframe`
    |
help: consider importing this struct
    |
1   + use crate::egui::ViewportBuilder;
    |
help: if you import `ViewportBuilder`, refer to it directly
    |
159 -         viewport: eframe::viewport::ViewportBuilder::default()
159 +         viewport: ViewportBuilder::default()
    |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `main` (bin "main") due to 1 previous error

## 2 [E0597]

error[E0597]: `audio_duration_secs` does not live long enough
   --> src/main.rs:174:54
    |
161 |     let audio_duration_secs = num_samples / sample_rate;
    |         ------------------- binding `audio_duration_secs` declared here
...
174 |         Box::new(|_cc| Box::new(AudioVisualizer::new(audio_duration_secs))),
    |         ---------------------------------------------^^^^^^^^^^^^^^^^^^^---
    |         |        |                                   |
    |         |        |                                   borrowed value does not live long enough
    |         |        value captured here
    |         cast requires that `audio_duration_secs` is borrowed for `'static`
...
178 | }
    | - `audio_duration_secs` dropped here while still borrowed
    |
    = note: due to object lifetime defaults, `Box<dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Box<dyn App>>` actually means `Box<(dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Box<dyn App> + 'static)>`

For more information about this error, try `rustc --explain E0597`.
error: could not compile `main` (bin "main") due to 1 previous error

