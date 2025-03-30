use std::collections::{HashMap, VecDeque};
use crate::output_handler::print_cli_line;
use crate::notes::frequency_to_note;
use crate::fft::fft_main::{note_to_freq, display_amplitude, NOTE_HISTORY, SPECTRUM_SCROLL};
use crate::output_handler::*;


pub fn analyze_frequencies(samples: &[f32]) -> (f32, f32, f32, String) {
    let mut low = 0.0;
    let mut mid = 0.0;
    let mut high = 0.0;
    let mut spectrum = Vec::new();


    let num_bins = 40;
    let min_freq = 20.0;
    let max_freq = 20000.0;
    let base = (max_freq / min_freq) as f32;
    let log_step = base.powf(1.0 / num_bins as f32);
    
    let mut bin_edges = Vec::with_capacity(num_bins + 1);
    let mut f = min_freq;
    for _ in 0..=num_bins {
        bin_edges.push(f);
        f *= log_step;
    }

    

    for (i, &sample) in samples.iter().enumerate() {
        let freq = (i as f32) * (44100.0 / samples.len() as f32);
        let magnitude = sample.abs();
        spectrum.push((freq, magnitude));

        if freq < 250.0 {
            low += magnitude;
        } else if freq < 4000.0 {
            mid += magnitude;
        } else {
            high += magnitude;
        }
    }

    spectrum.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut bins = vec![0.0f32; num_bins];

    let max_height = 8;
    let mut columns: Vec<Vec<char>> = vec![vec![' '; max_height]; num_bins];

    for (i, amp) in bins.iter().enumerate() {
        let height = (amp * max_height as f32 * 10.0).clamp(0.0, max_height as f32) as usize;
        for h in 0..height {
            columns[i][max_height - 1 - h] = match amp {
                a if *a > 0.1 => '█',
                a if *a > 0.05 => '▓',
                a if *a > 0.01 => '▒',
                a if *a > 0.001 => '░',
                _ => ' ',
            };
        }
    }

    // print!("\x1B[2J\x1B[H"); // clear + home

    // for row in 0..max_height {
        // for col in 0..num_bins {
            // print!("{}", columns[col][row]);
        // }
        // println!();
    // }
    // println!("+{}+", "-".repeat(num_bins));


    for (f, a) in spectrum.iter() {
        if *f < min_freq || *f > max_freq { continue; }
        let bin_idx = bin_edges.iter().position(|edge| *f < *edge).unwrap_or(num_bins) - 1;
        bins[bin_idx] += *a;
    }

    let line = bins.iter().map(|amp| {
        match amp {
            a if *a > 0.1 => '█',
            a if *a > 0.05 => '▓',
            a if *a > 0.01 => '▒',
            a if *a > 0.001 => '░',
            _ => ' ',
        }
    }).collect::<String>();
    
    // print!("\r{}", line);

    use crate::ascii_visual::render_vertical_eq;

    let eq_visual = render_vertical_eq(&bins, 12);
    // print!("\x1B[2J\x1B[H{}", eq_visual);


    // unsafe {
    //     if SPECTRUM_SCROLL.is_none() {
    //         SPECTRUM_SCROLL = Some(VecDeque::with_capacity(20));
    //     }

    //     if let Some(scroll) = SPECTRUM_SCROLL.as_mut() {
    //         if scroll.len() >= 20 {
    //             scroll.pop_front();
    //         }
    //         scroll.push_back(line.clone());

    //         print!("\x1B[2J\x1B[H"); // clear screen + move to top
    //         for l in scroll.iter() {
    //             println!("{}", l);
    //         }
    //     }
    // }

    

    let total_energy: f32 = spectrum.iter().map(|(_, amp)| amp).sum();
    if total_energy < 0.01 {
        return (low, mid, high, String::new());
    }
    
    // .filter(|(f, _)| *f < 24000.0)

    let debug_freqs = spectrum
        .iter()
        .filter(|(f, _)| *f < 24000.0)
        .take(5)
        .map(|(f, a)| format!("{:>7.1}Hz:{:.5}", f, a))
        .collect::<Vec<_>>()
        .join(" ");

    
    let debug_line = format!("🎯 {}", debug_freqs);


    static mut FRAME_COUNT: usize = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 10 == 0 {
            unsafe {
                if NOTE_HISTORY.is_none() {
                    NOTE_HISTORY = Some(VecDeque::with_capacity(10));
                }
                
                let mut note_groups: HashMap<String, Vec<i32>> = HashMap::new();
            
                if let Some(history) = NOTE_HISTORY.as_mut() {
                    for (freq, amp) in spectrum.iter().take(10) {
                        if *amp > 0.0001 && *freq < 20000.0 {
                            let note = frequency_to_note(*freq);
                            if note == "Unknown" {
                                continue;
                            }
                    
                            let base_freq = note_to_freq(&note);
                            if base_freq <= 0.0 {
                                continue;
                            }
                    
                            let cents = 1200.0 * (freq / base_freq).log2();
                            if cents.abs() > 200.0 {
                                continue;
                            }
                    
                            note_groups
                                .entry(note)
                                .or_default()
                                .push(cents.round() as i32);
                        }
                    }
                    
                    let mut display_line = String::from("🎯 ");
                    
                    for (note, cents_list) in note_groups {
                        let color = match cents_list.iter().map(|c| c.abs()).max().unwrap_or(0) {
                            0..=5 => "green",
                            6..=15 => "yellow",
                            _ => "red",
                        };
                    
                        let cents_str = cents_list
                            .iter()
                            .map(|c| format!("{:+}c", c))
                            .collect::<Vec<_>>()
                            .join(", ");
                    
                            display_line += &format!("{} ({}) ", note, cents_str);
                        }

                        // print_cli_line(&display_line);
                }
            }
        }
    }

    // display_amplitude(low, mid, high);
    (low, mid, high, debug_line)
}

