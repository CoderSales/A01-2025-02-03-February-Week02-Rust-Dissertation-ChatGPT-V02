# Chat

## In

### CL Output

```bash
error[E0407]: method `name` is not a member of trait `eframe::App`
  --> src\gui.rs:16:5
   |
16 | /     fn name(&self) -> &str {
17 | |         "Audio Frequency Analyzer"
18 | |     }
   | |_____^ not a member of trait `eframe::App`

error[E0407]: method `name` is not a member of trait `eframe::App`
  --> src\gui.rs:60:5
   |
60 | /     fn name(&self) -> &str {
61 | |         "Frequency Meter"
62 | |     }
   | |_____^ not a member of trait `eframe::App`

error[E0061]: this method takes 0 arguments but 1 argument was supplied
  --> src\gui.rs:71:48
   |
71 |             ui.add(egui::ProgressBar::new(low).show_percentage(false));
   |                                                ^^^^^^^^^^^^^^^ ----- unexpected argument of type `bool`
   |
note: method defined here
  --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\egui-0.31.0\src\widgets\progress_bar.rs:69:12
   |
69 |     pub fn show_percentage(mut self) -> Self {
   |            ^^^^^^^^^^^^^^^
help: remove the extra argument
   |
71 -             ui.add(egui::ProgressBar::new(low).show_percentage(false));
71 +             ui.add(egui::ProgressBar::new(low).show_percentage());
   |

error[E0061]: this method takes 0 arguments but 1 argument was supplied
  --> src\gui.rs:73:48
   |
73 |             ui.add(egui::ProgressBar::new(mid).show_percentage(false));
   |                                                ^^^^^^^^^^^^^^^ ----- unexpected argument of type `bool`
   |
note: method defined here
  --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\egui-0.31.0\src\widgets\progress_bar.rs:69:12
   |
69 |     pub fn show_percentage(mut self) -> Self {
   |            ^^^^^^^^^^^^^^^
help: remove the extra argument
   |
73 -             ui.add(egui::ProgressBar::new(mid).show_percentage(false));
73 +             ui.add(egui::ProgressBar::new(mid).show_percentage());
   |

error[E0061]: this method takes 0 arguments but 1 argument was supplied
  --> src\gui.rs:75:49
   |
75 |             ui.add(egui::ProgressBar::new(high).show_percentage(false));
   |                                                 ^^^^^^^^^^^^^^^ ----- unexpected argument of type `bool`
   |
note: method defined here
  --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\egui-0.31.0\src\widgets\progress_bar.rs:69:12
   |
69 |     pub fn show_percentage(mut self) -> Self {
   |            ^^^^^^^^^^^^^^^
help: remove the extra argument
   |
75 -             ui.add(egui::ProgressBar::new(high).show_percentage(false));
75 +             ui.add(egui::ProgressBar::new(high).show_percentage());
   |

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> src\gui.rs:104:9
    |
104 |         eframe::run_native(Box::new(|_| Box::new(app)), options);
    |         ^^^^^^^^^^^^^^^^^^-------------------------------------- argument #3 of type `Box<dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>>` is missing
    |
note: expected `&str`, found `Box<{closure@gui.rs:104:37}>`
   --> src\gui.rs:104:28
    |
104 |         eframe::run_native(Box::new(|_| Box::new(app)), options);
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected reference `&str`
                  found struct `Box<{closure@src\gui.rs:104:37: 104:40}>`
note: function defined here
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\eframe-0.31.0\src/lib.rs:240:8
    |
240 | pub fn run_native(
    |        ^^^^^^^^^^
help: provide the argument
    |
104 |         eframe::run_native(/* &str */, options, /* Box<dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>> */);
    |                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> src\gui.rs:107:5
    |
107 |     eframe::run_native(Box::new(|_| Box::new(freq_meter)), options);
    |     ^^^^^^^^^^^^^^^^^^--------------------------------------------- argument #3 of type `Box<dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>>` is missing
    |
note: expected `&str`, found `Box<{closure@gui.rs:107:33}>`
   --> src\gui.rs:107:24
    |
107 |     eframe::run_native(Box::new(|_| Box::new(freq_meter)), options);
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected reference `&str`
                  found struct `Box<{closure@src\gui.rs:107:33: 107:36}>`
note: function defined here
   --> C:\Users\steph\.cargo\registry\src\index.crates.io-6f17d22bba15001f\eframe-0.31.0\src/lib.rs:240:8
    |
240 | pub fn run_native(
    |        ^^^^^^^^^^
help: provide the argument
    |
107 |     eframe::run_native(/* &str */, options, /* Box<dyn for<'a, 'b> FnOnce(&'a CreationContext<'b>) -> Result<Box<dyn App>, Box<(dyn std::error::Error + Send + Sync + 'static)>>> */);
    |                       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0308]: mismatched types
  --> src/main.rs:21:12
   |
21 |     if let Err(e) = gui::launch_gui() {
   |            ^^^^^^   ----------------- this expression has type `()`
   |            |
   |            expected `()`, found `Result<_, _>`
   |
   = note: expected unit type `()`
                   found enum `Result<_, _>`

warning: unused variable: `frame`
  --> src\gui.rs:20:47
   |
20 |     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
   |                                               ^^^^^ help: if this is intentional, prefix it with an underscore: `_frame`
   |
   = note: `#[warn(unused_variables)]` on by default

Some errors have detailed explanations: E0061, E0308, E0407.
For more information about an error, try `rustc --explain E0061`.
warning: `midi_freq_analyzer` (bin "midi_freq_analyzer") generated 1 warning
error: could not compile `midi_freq_analyzer` (bin "midi_freq_analyzer") due to 8 previous errors; 1 warning emitted
```
