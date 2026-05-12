pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
