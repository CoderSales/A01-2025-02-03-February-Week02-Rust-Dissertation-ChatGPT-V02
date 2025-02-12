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

## 3 [E0599]

error[E0599]: no method named `convert_samples` found for struct `Decoder` in the current scope
   --> src/main.rs:21:43
    |
21  |     let _ = stream_handle.play_raw(source.convert_samples());
    |                                           ^^^^^^^^^^^^^^^
    |
   ::: C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\rodio-0.17.3\src\source\mod.rs:300:8
    |
300 |     fn convert_samples<D>(self) -> SamplesConverter<Self, D>
    |        --------------- the method is available for `Decoder<BufReader<File>>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `Source` which provides `convert_samples` is implemented but not in scope; perhaps you want to import it
    |
1   + use rodio::Source;
    |
help: there is a method `into_sample` with a similar name
    |
21  |     let _ = stream_handle.play_raw(source.into_sample());
    |                                           ~~~~~~~~~~~

For more information about this error, try `rustc --explain E0599`.
