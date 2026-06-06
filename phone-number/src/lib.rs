pub fn number(user_number: &str) -> Option<String> {
    // parse digits excluding valid separator chars
    let digits = user_number
        .chars()
        .filter(|&c| ![' ', '-', '.', '(', ')', '+'].contains(&c))
        .map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Option<Vec<u8>>>();

    digits.and_then(|n| {
        // skip first digit if it's +1 area code
        let slice = if n.len() == 11 && n[0] == 1 {
            &n[1..]
        } else {
            &n
        };

        // check total length,  1st and 4th digits to be in [2..9] range, return result
        (slice.len() == 10 && slice[0] >= 2 && slice[3] >= 2)
            .then(|| slice.iter().map(|d| d.to_string()).collect())
    })
}
