pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&['-', ' '])
        .flat_map(|word| {
            word.chars().filter(|c| c.is_alphabetic()).take(1).chain(
                word.chars()
                    .filter(|c| c.is_alphabetic())
                    .skip_while(|&ch| ch.is_uppercase())
                    .filter(|&ch| ch.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
