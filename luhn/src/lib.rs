/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut has_invalid_characters = false;
    let mut count = 0;

    let checksum: u32 = code
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .map(|c| {
            if let Some(c) = c {
                count += 1;
                if count % 2 == 1 { return c; }
                if c > 4 { 2 * c - 9 } else { 2 * c }
            } else {
                has_invalid_characters = true;
                0
            }
        })
        .sum();

    checksum.is_multiple_of(10) && !has_invalid_characters && count > 1
}
