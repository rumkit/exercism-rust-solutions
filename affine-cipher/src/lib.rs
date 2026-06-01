/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_LEN: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a,ALPHABET_LEN) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        let result = plaintext.chars()
            .filter(|&x| x.is_ascii_alphabetic() | x.is_ascii_digit())
            .map(|c| {
                match c.to_ascii_lowercase() {
                    '0'..='9' => c,
                    c => {
                        (b'a' + ((a * (c as i32 - b'a' as i32) + b) % ALPHABET_LEN) as u8) as char
                    }
                }
            })
            .collect::<Vec<_>>()
            .chunks(5)
            .collect::<Vec<_>>()
            .join(&' ')
            .into_iter()
            .collect::<String>();

        Ok(result)
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a,ALPHABET_LEN) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        let result = ciphertext.chars()
            .filter(|&x| x.is_ascii_alphabetic() | x.is_ascii_digit())
            .map(|c| {
                match c.to_ascii_lowercase() {
                    '0'..='9' => c,
                    c => {
                        let mmi = mod_inverse(a, ALPHABET_LEN).unwrap();
                        let t = (c as u8 - b'a') as i32 - b;
                        (b'a' + ((mmi * t ).rem_euclid(ALPHABET_LEN)) as u8) as char
                    }
                }
            })
            .collect::<String>();

        Ok(result)
    }
}

/// Computes the Greatest Common Divisor (GCD) and coefficients (x, y)
/// such that: a*x + b*y = gcd(a, b)
fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

/// Finds the modular multiplicative inverse of `a` modulo `m`
/// Returns `Some(inverse)` if it exists, otherwise `None`
fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    if m <= 0 {
        return None; // modulus must be positive
    }
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        None // inverse does not exist if gcd(a, m) != 1
    } else {
        // Ensure the result is positive
        Some((x % m + m) % m)
    }
}

fn is_coprime(a: i32, b: i32) -> bool {
    extended_gcd(a,b).0 == 1
}
