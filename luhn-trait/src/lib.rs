pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let input = self.to_string();
        let count = input.len();

        let checksum: Option<u32> = input
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .map(|(i, c)| {
                c.to_digit(10)
                    .map(| c| {
                        if i % 2 == 0 { return c; }
                        if c > 4 { 2 * c - 9 } else { 2 * c }
                    })
            })
            .sum();

        checksum.is_some_and( |c| c.is_multiple_of(10)) && count > 1
    }
}