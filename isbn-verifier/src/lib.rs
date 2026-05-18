/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars = isbn.chars().filter(|&c| c != '-').collect::<Vec<char>>();

    if chars.len() != 10 || chars[..9].iter().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    let sum = chars[..9].iter()
        .rev()
        .enumerate()
        .fold(0,
              |acc, (i,b)|
                  (acc + (*b as u8  - b'0') * (i as u8 + 2)) % 11);

    if sum == 1 {
        chars[9] == 'X'
    }
    else {
        chars[9] as u8 - b'0' == (11 - sum % 11) % 11
    }
}
