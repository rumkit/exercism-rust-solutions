use std::collections::HashMap;
use std::iter;

use itertools::Itertools;


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // split two sides of equation
    let (addends, result_sum) = input.split_once(" == ").unwrap();
    // transform each number to a Vec<u8> with A-based indexes (ABDK -> 1,2,4,10)
    // first comes the result_sum, then all addends
    let numbers = iter::once(result_sum)
        .chain(addends.split(" + "))
        .map(|word| word.chars().map(|c| (c as u8) - b'A').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // flatten and sort all unique letters
    let mut unique = numbers.iter().flatten().copied().unique().collect::<Vec<_>>();
    unique.sort_unstable();

    // using array as a simple letter-value map
    let mut assignment = [0; 26];
    for perm in (0..10).permutations(unique.len()) {
        // assign mapping for the current permutation
        for (&letter, value) in unique.iter().zip(perm) {
            assignment[letter as usize] = value;
        }

        // drop solution if there is a leading zero
        if numbers.iter().any(|n| assignment[n[0] as usize] == 0) {
            continue;
        }

        let sum: u64 = numbers[1..].iter().map(|n| letters_to_number(n, &assignment)).sum();
        let solution = letters_to_number(&numbers[0], &assignment);

        if sum == solution {
            return Some(
                unique
                    .into_iter()
                    .map(|c| ((c + b'A') as char, assignment[c as usize]))
                    .collect(),
            );
        }
    }

    None
}

// using provided mapping, convert letters to a real number
fn letters_to_number(letters: &[u8], assignment: &[u8; 26]) -> u64 {
    letters.iter().fold(0, |acc, next| {
        acc * 10 + (assignment[*next as usize] as u64)
    })
}
