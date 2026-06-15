#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

// Using byte slice formatting for safe, O(1) matching without char-boundary panic risks
static DIGITS: &[&[u8]] = &[
    b" _     _  _     _  _  _  _  _ ",
    b"| |  | _| _||_||_ |_   ||_||_|",
    b"|_|  ||_  _|  | _||_|  ||_| _|",
    b"                              ",
];

pub fn convert(input: &str) -> Result<String, Error> {
    // Process input as byte slices instead of strings
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    // 1. Validate column counts
    if let Some(faulty) = lines.iter().find(|line| line.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(faulty.len()));
    }

    // 2. Validate row counts using stable slice::as_chunks
    let (chunks, remainder) = lines.as_chunks::<4>();
    if !remainder.is_empty() {
        return Err(Error::InvalidRowCount(remainder.len()));
    }

    // 3. Process the validated chunks
    let result = chunks
        .iter()
        .map(|chunk| {
            let cols = chunk[0].len();
            // Pre-allocate memory since we know exactly how many digits to expect
            let mut group = String::with_capacity(cols / 3);

            for i in (0..cols).step_by(3) {
                group.push(match_digit(chunk, i));
            }
            group
        })
        .collect::<Vec<String>>()
        .join(",");

    Ok(result)
}

fn match_digit(strip: &[&[u8]], from: usize) -> char {
    let to = from + 3;

    // Explicitly check for digits 0-9
    (0..10)
        .find(|&d| {
            let offset = d * 3;
            (0..4).all(|row| strip[row][from..to] == DIGITS[row][offset..offset + 3])
        })
        .map(|d| (b'0' + d as u8) as char)
        .unwrap_or('?')
}