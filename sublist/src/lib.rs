#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        ([], []) => Comparison::Equal,
        (_, []) => Comparison::Superlist,
        ([], _) => Comparison::Sublist,
        (first_list, second_list) => {
            let (x, y) = if first_list.len() > second_list.len() {
                (first_list, second_list)
            } else {
                (second_list, first_list)
            };

            let mut has_subsequence = false;
            for i in 0..=x.len() - y.len() {
                if try_find_subsequence(x, y, i) {
                    has_subsequence = true;
                    break;
                }
            }

            match (has_subsequence, first_list.len(), second_list.len()) {
                (true, first, second) if first < second => Comparison::Sublist,
                (true, first, second) if first > second => Comparison::Superlist,
                (true, first, second) if first == second => Comparison::Equal,
                _ => Comparison::Unequal,
            }
        }
    }
}

fn try_find_subsequence(first: &[i32], second: &[i32], start_from: usize) -> bool {
    for i in start_from..second.len() {
        if first[i] != second[i - start_from] {
            return false;
        }
    }

    true
}
