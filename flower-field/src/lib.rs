#[rustfmt::skip]
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),           ( 0, 1),
    ( 1, -1),  ( 1, 0), ( 1, 1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for i in 0..garden.len() {
        let mut output_row = String::new();
        for j in 0..garden[i].len() {
            if garden[i].as_bytes()[j] == b'*' {
                output_row.push('*');
                continue;
            }
            let adjacent_count = DIRECTIONS
                .iter()
                .flat_map(|&(dx, dy)| {
                    garden
                        .get((i as i32 + dy) as usize)
                        .and_then(|&row| row.as_bytes().get((j as i32 + dx) as usize))
                })
                .filter(|&&c| c == b'*')
                .count() as u8;
            if adjacent_count > 0 {
                output_row.push((adjacent_count + 0x30) as char);
            }
            else {
                output_row.push(' ');
            }
        }
        output.push(output_row);
    }

    output
}
