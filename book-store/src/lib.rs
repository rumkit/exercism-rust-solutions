use std::cmp::min;
use std::collections::HashMap;

const DISCOUNTS: [u32;5] = [0, 5, 10, 20, 25];

pub fn lowest_price(books: &[u32]) -> u32 {
    let book_counts = books.iter().fold(HashMap::new(), |mut map, book| {
        map.entry(book).and_modify(|v| *v += 1).or_insert(1);
        map
    });

    let mut group_counts = book_counts.into_values().collect::<Vec<_>>();

    // convert book counts to counts of groups of (i + 1)
    group_counts.sort_unstable();
    group_counts.reverse();
    for i in 0.. group_counts.len().saturating_sub(1) {
        group_counts[i] -= group_counts[i+1];
    }

    // convert each 3 + 5 to 2 groups of four
    if group_counts.len() == 5 {
        let to_fours = min(group_counts[3 - 1], group_counts[5 - 1]);
        group_counts[3 - 1] -= to_fours;
        group_counts[5 - 1] -= to_fours;
        group_counts[4 - 1] += to_fours * 2;
    }

    // calculate total
    group_counts.into_iter().enumerate().zip(DISCOUNTS.iter()).fold(0, |total, ((g, count), discount)|
        total + count * (g as u32 + 1) * 8 * (100 - discount))
}
