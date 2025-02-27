# Chat

## Out

Fix for Move Errors (E0382)
1️⃣ Error: options moved
options is moved into the first eframe::run_native() call, making it unavailable for the second one.
✅ Solution: Clone options before the second call:

rust
Copy
Edit
let options_clone = options.clone();
Then use options_clone in the second eframe::run_native() call.

2️⃣ Error: freq_meter moved
freq_meter is moved inside the first closure, making it unavailable for the second one.
✅ Solution 1 (Preferred): Clone freq_meter If FrequencyMeter implements Clone, clone it before passing it:

rust
Copy
Edit
let freq_meter_clone = freq_meter.clone();
Use freq_meter_clone in the second eframe::run_native().

✅ Solution 2 (Alternative): Use Arc<Mutex<FrequencyMeter>>

If cloning isn't feasible, wrap it in Arc<Mutex<...>> for shared access.
Final Fixed Code
Modify launch_gui():

rust
Copy
Edit
pub fn launch_gui() {
    let options = eframe::NativeOptions::default();
    let options_clone = options.clone(); // Clone before second use

    let log_output = Arc::new(Mutex::new(String::new()));
    let low_freq = Arc::new(Mutex::new(0.0));
    let mid_freq = Arc::new(Mutex::new(0.0));
    let high_freq = Arc::new(Mutex::new(0.0));

    let app = AudioApp {
        status_message: "Idle".to_string(),
        log_output: Arc::clone(&log_output),
        low_freq: Arc::clone(&low_freq),
        mid_freq: Arc::clone(&mid_freq),
        high_freq: Arc::clone(&high_freq),
    };

    let freq_meter = FrequencyMeter {
        low_freq,
        mid_freq,
        high_freq,
    };

    let freq_meter_clone = freq_meter.clone(); // Clone before second use

    thread::spawn(move || {
        if let Err(e) = eframe::run_native(
            "Audio Analyzer",
            options,
            Box::new(|_cc| Ok(Box::new(app))),
        ) {
            eprintln!("Error launching Audio Analyzer: {:?}", e);
        }
    });

    thread::sleep(Duration::from_millis(100)); // Prevent UI conflict

    if let Err(e) = eframe::run_native(
        "Frequency Meter",
        options_clone,
        Box::new(|_cc| Ok(Box::new(freq_meter_clone))),
    ) {
        eprintln!("Error launching Frequency Meter: {:?}", e);
    }
}
Next Steps
Apply the fix.
Run cargo check to verify no errors.
Run cargo run to confirm two GUI windows open.
✅ This ensures both GUIs launch independently without ownership issues! 🚀

## In

