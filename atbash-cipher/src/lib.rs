/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
        .filter(|&x| x.is_ascii_alphabetic() | x.is_ascii_digit())
        .map(map_char)
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i != 0 && i % 5 == 0 {
                acc.push(' ');
            }
            acc.push(c);
            acc
        }).trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter(|&x| x != ' ')
        .map(map_char)
        .collect()
}

fn map_char(c: char) -> char {
    let c = c.to_ascii_lowercase();
    match c.is_alphabetic() {
        true => {
            (b'a' + (b'z' - (c as u8))) as char
        },
        false => c,
    }
}
