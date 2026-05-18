pub fn check(candidate: &str) -> bool {
    let mut letters = [false; 26];
    for c in candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
    {
        if letters[c as usize - 'a' as usize] {
            return false;
        }
        letters[c as usize - 'a' as usize] = true;
    }

    true
}
