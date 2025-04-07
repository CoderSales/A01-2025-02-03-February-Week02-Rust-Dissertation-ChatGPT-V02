// src/analytics/note_label.rs

pub fn frequency_to_note(frequency: f32) -> String {
    if frequency <= 0.0 {
        return "---".to_string();
    }

    let a4 = 440.0;
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let n = (12.0 * (frequency / a4).log2()).round() as i32 + 69;
    let note_index = n.rem_euclid(12) as usize;
    let note_name = note_names[note_index];
    format!("{} ({:.1} Hz)", note_name, frequency)
}
