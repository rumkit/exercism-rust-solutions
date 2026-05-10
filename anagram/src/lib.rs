use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut confirmed_anagrams = HashSet::new();
    let orig_map = create_chars_map(word);

    for &p in possible_anagrams {
        if is_confirm_anagram(&orig_map, p) && p.to_lowercase() != word.to_lowercase() {
            confirmed_anagrams.insert(p);
        }
    }

    confirmed_anagrams
}

fn create_chars_map(word: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in word.to_lowercase().chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    map
}

fn is_confirm_anagram(original: &HashMap<char, i32>, candidate: &str) -> bool {
    let candidate_map = create_chars_map(candidate);
    *original == candidate_map
}