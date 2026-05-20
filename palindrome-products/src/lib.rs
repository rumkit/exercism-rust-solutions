use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    factors: HashSet<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 { self.value }

    pub fn into_factors(self) -> HashSet<(u64, u64)> { self.factors }

    pub fn new(value: u64) -> Self { Palindrome { factors: HashSet::new(), value}}
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut palindromes: HashMap<u64, Palindrome> = HashMap::new();
    let mut min_pal = None;
    let mut max_pal = None;

    let length = max - min;
    for i in 0..=length {
        for j in i..=length {
            let x = i + min;
            let y = j + min;
            let candidate = x*y;
            if is_palindrome(candidate) {
                if min_pal.is_none() || min_pal.unwrap() > candidate {
                    min_pal = Some(candidate);
                }
                if max_pal.is_none() || max_pal.unwrap() < candidate {
                    max_pal = Some(candidate);
                }
                palindromes.entry(candidate).or_insert(Palindrome::new(candidate)).factors.insert((x, y));
            }
        }
    }

    match (min_pal, max_pal) {
        (Some(min), Some(max)) => Some((palindromes[&min].clone(), palindromes[&max].clone())),
        _ => None
    }
}

fn is_palindrome(number: u64) -> bool {
    let mut reversed = 0;
    let mut n = number;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    reversed == number
}
