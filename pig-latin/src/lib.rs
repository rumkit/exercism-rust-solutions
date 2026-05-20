pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn translate_word(input: &str) -> String {
    let cs: Vec<char> = input.chars().collect();

    // Rule 1: starts with vowel or "xr"/"yt"
    if is_vowel(&cs[0]) || cs.starts_with(&['x', 'r']) || cs.starts_with(&['y', 't']) {
        return format!("{input}ay");
    }

    // Rule 3: 0 or more consonants + "qu"
    if let Some(i) = find_qu_position(&cs) {
        let (h, t) = cs.split_at(i + 2);
        return format!(
            "{}{}ay",
            t.iter().collect::<String>(),
            h.iter().collect::<String>()
        );
    }

    // Rule 4: 1 or more consonants + 'y'
    if let Some(i) = find_y_position(&cs) && i > 0
    {
        let (h, t) = cs.split_at(i);
        return format!(
            "{}{}ay",
            &t.iter().collect::<String>(),
            &h.iter().collect::<String>()
        );
    }

    // Rule 2: consonant prefix
    let i = cs.iter().position(is_vowel).unwrap_or(cs.len());
    let (h, t) = cs.split_at(i);
    format!(
        "{}{}ay",
        t.iter().collect::<String>(),
        h.iter().collect::<String>()
    )
}

fn find_qu_position(cs: &[char]) -> Option<usize> {
    cs.windows(2)
        .position(|w| w == ['q', 'u'])
        .filter(|&i| cs[..i].iter().all(is_consonant))
}

fn find_y_position(cs: &[char]) -> Option<usize> {
    cs.iter()
        .position(|&c| c == 'y')
        .filter(|&i| cs[..i].iter().all(is_consonant))
}

fn is_vowel(character: &char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(character)
}

fn is_consonant(character: &char) -> bool { !is_vowel(character) }
