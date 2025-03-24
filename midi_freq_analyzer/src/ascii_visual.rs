pub fn render_vertical_eq(bins: &[f32], max_height: usize) -> String {
    let mut columns = vec![vec![' '; max_height]; bins.len()];
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

    let mut lines = Vec::new();
    for row in 0..max_height {
        let line: String = columns.iter().map(|col| col[row]).collect();
        lines.push(line);
    }
    lines.push(format!("+{}+", "-".repeat(bins.len())));
    lines.join("\n")
}
