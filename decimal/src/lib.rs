use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    value: BigInt,
    exponent: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let exponent = input
            .bytes()
            .rev()
            .position(|b| b == b'.')
            .unwrap_or(0);
        let value = input.replace('.', "").parse().ok()?;
        Some(Self { value, exponent: exponent as u32 }.normalize_exponent())
    }
}

impl Decimal {
    fn match_exp_with(&mut self, other: &mut Self) {
        if self.exponent < other.exponent {
            self.value *= BigInt::from(10).pow(other.exponent - self.exponent);
            self.exponent = other.exponent;
        } else {
            other.value *= BigInt::from(10).pow(self.exponent - other.exponent);
            other.exponent = self.exponent;
        }
    }

    fn normalize_exponent(mut self) -> Self {
        if self.value == 0.into() {
            self.exponent = 0;
        }
        while self.exponent > 0 && self.value.clone() % 10 == 0.into() {
            self.value /= 10;
            self.exponent -= 1;
        }

        self
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self::Output {
        self.match_exp_with(&mut other);
        self.value += other.value;
        self.normalize_exponent()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(mut self, mut other: Self) -> Self::Output {
        self.match_exp_with(&mut other);
        self.value -= other.value;
        self.normalize_exponent()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self::Output {
        self.value *= other.value;
        self.exponent += other.exponent;
        self.normalize_exponent()
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut left = self.clone();
        let mut right = other.clone();
        left.match_exp_with(&mut right);

        left.value.partial_cmp(&right.value)
    }
}