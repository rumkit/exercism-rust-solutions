#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = (1..num).filter( |&x| num.is_multiple_of(x)).sum();
    if sum == num {
        Some(Classification::Perfect)
    }
    else if sum < num {
        Some(Classification::Deficient)
    }
    else {
        Some(Classification::Abundant)
    }
}
