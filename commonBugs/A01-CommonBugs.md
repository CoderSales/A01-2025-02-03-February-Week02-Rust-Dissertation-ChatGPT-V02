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

## 2