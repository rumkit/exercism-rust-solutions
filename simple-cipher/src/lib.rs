const MOD: u8 = b'z' - b'a' + 1;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| char::is_ascii_lowercase(&c)) {
        return None;
    }

    let result = s.chars()
        .zip(key.chars().cycle())
        .map(|(c, key_part) | {
            let key_byte = key_part as u8 - b'a';
            match c {
                'A'..='Z' => (b'A' + (c as u8 - b'A' + key_byte) % MOD) as char,
                'a'..='z' => (b'a' + (c as u8 - b'a' + key_byte) % MOD) as char,
                _ => c,
            }})
        .collect::<String>();

    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| char::is_ascii_lowercase(&c)) {
        return None;
    }
    let key_complement = key.as_bytes()
        .iter()
        .map(|b| {
            (b'a' + (MOD - (b - b'a')) % MOD) as char
        })
        .collect::<String>();

    encode(&key_complement, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    const RANDOM_LEN: usize = 100;
    let mut key = String::with_capacity(RANDOM_LEN);

    for _ in 0..RANDOM_LEN {
        key.push(rand::random_range('a'..='z'));
    }

    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}
