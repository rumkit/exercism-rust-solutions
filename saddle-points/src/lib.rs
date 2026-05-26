use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_in_row: HashMap<usize, u64> = HashMap::new();
    let mut answers: Vec<(usize, usize)> = Vec::new();

    for column in 0..input[0].len() {
        let smallest_in_column = input.iter().map(|row| row[column]).min().unwrap();

        for (row, _) in input
            .iter()
            .enumerate()
            .filter(|&(_, row)| row[column] == smallest_in_column)
        {
            let max = max_in_row
                .entry(row)
                .or_insert_with(|| input[row].iter().copied().max().unwrap());

            if *max == smallest_in_column {
                answers.push((row, column));
            }
        }
    }

    answers
}
