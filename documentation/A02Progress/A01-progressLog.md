# Progress Log

## Output

Sample Rate: 48000
Bits per Sample: 16
Channels: 2
Loaded 1156800 samples

## Added

winapi = { version = "0.3", features = ["winuser"] }

## Error 2

error[E0432]: unresolved import eframe::egui::plot
  --> src/main.rs:11:26
   |
11 | use eframe::{egui, egui::plot::{Plot, Line, Values}};
   |                          ^^^^ could not find plot in egui

For more information about this error, try rustc --explain E0432.
error: could not compile main (bin "main") due to 1 previous error

