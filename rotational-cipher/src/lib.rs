pub fn rotate(input: &str, key: u8) -> String {
    const MOD:u8 = b'z' - b'a' + 1;
    input
        .chars()
        .map(|c| {
            match c {
                'A'..='Z' => (b'A' + (c as u8 - b'A' + key) % MOD) as char,
                'a'..='z' => (b'a' + (c as u8 - b'a' + key) % MOD) as char,
                _ => c,
            }
        })
        .collect::<String>()
}
