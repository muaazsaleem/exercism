pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() || garden[0].is_empty() {
        return garden.iter().map(|_| String::new()).collect();
    }
    for_each_cell(garden, |row, col, byte| {
        if byte == b'*' {
            '*'
        } else {
            as_char(neighbor_count(row, col, garden))
        }
    })
}

fn for_each_cell(garden: &[&str], c: impl Fn(usize, usize, u8) -> char) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, cols)| {
            cols.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, &byte)| c(row, col, byte))
                .collect()
        })
        .collect()
}

fn neighbor_count(row: usize, col: usize, garden: &[&str]) -> u8 {
    let height = garden.len();
    let width = garden[0].len();
    // slice a 3x3 patch around (row, col), clamped to garden bounds
    garden[row.saturating_sub(1)..(row + 2).min(height)]
        .iter()
        .flat_map(|rstr| rstr[col.saturating_sub(1)..(col + 2).min(width)].bytes())
        .filter(|&byte| byte == b'*')
        .count() as u8
}

fn as_char(count: u8) -> char {
    if count == 0 {
        ' '
    } else {
        (b'0' + count) as char
    }
}