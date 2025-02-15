/// Live amplitude visualization using `_` for simple horizontal bars
pub fn print_live_amplitude(amplitude: f32) {
    let level = (amplitude * 50.0) as usize; // Scale amplitude
    let bar = "_".repeat(level); // Create bar of `_`
    println!("\r[{}]", bar); // Print inline
}
