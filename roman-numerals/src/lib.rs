use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", convert_to_roman(self.0))
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

fn convert_to_roman(mut num: u32) -> String {
    let mut output = String::new();
    while num > 0 {
        for &(value, literal) in ROMANS.iter() {
            while num >= value {
                num -= value;
                output.push_str(literal);
            }
        }
    }

    output
}

const ROMANS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];