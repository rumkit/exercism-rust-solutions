#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    // parse digits to vec of u64
    let digits = string_digits.chars()
        .map(|c| c.to_digit(10).map(|d| d as u64).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, _>>()?;
    // initial span
    let mut max_product:u64 = 0;
    let mut product = digits.iter().take(span).filter(|&&d| d != 0).product();
    let mut zero_count = digits.iter().take(span).filter(|&&d| d == 0).count();
    if zero_count == 0 {
        max_product = product;
    }
    // sliding window
    for i in span..string_digits.len() {
        let next = digits[i];
        let last = digits[i - span];
        if last != 0 {
            product /= last;
        } else {
            zero_count -= 1;
        }
        if next != 0 {
            product *= next;
        } else {
            zero_count += 1;
        }
        if zero_count == 0 {
            max_product = max_product.max(product);
        }
    }

    Ok(max_product)
}
