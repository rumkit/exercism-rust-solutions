pub struct Luhn {
    digits_valid: bool,
    digits: Vec<u8>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if !self.digits_valid {
            return false;
        }

        let count = self.digits.len();
        let checksum = self.digits.iter()
            .enumerate()
            .map(|(i, digit)| {
                let dig = *digit as u32;
                if i % 2 == 0 {
                    return dig;
                }
                if dig > 4 { 2 * dig - 9 } else { 2 * dig }
            })
            .sum::<u32>();
        count > 1 && checksum.is_multiple_of(10)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let mut digits_valid = true;
        let digits = input
            .to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                c.to_digit(10).map(|n| n as u8).unwrap_or_else(|| {
                    digits_valid = false;
                    0
                })
            })
            .collect();
        Self {
            digits,
            digits_valid,
        }
    }
}
