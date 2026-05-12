pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut number = num;
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }

    digits
        .iter()
        .map(|x| x.pow(digits.len() as u32))
        .sum::<u32>() == num
}
