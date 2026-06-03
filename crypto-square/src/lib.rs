pub fn encrypt(input: &str) -> String {
    // input string normalization
    let normalized_string = input.chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();

    let (n_cols, n_rows) = get_square_size(normalized_string.len());

    // building square as vec of columns
    let mut encoded = vec![];
    for i in 0..n_cols {
        let mut next_column = normalized_string
            .char_indices()
            .filter(|&(index, _)| index % n_cols == i)
            .map(|(_, c)| c)
            .collect::<String>();
        // append padding if required
        if next_column.len() != n_rows {
            next_column.push(' ');
        }
        encoded.push(next_column);
    }

    encoded.join(" ")
}

pub fn get_square_size(length: usize) -> (usize, usize) {
    let columns = (length as f64).sqrt().ceil();
    let rows = (length as f64 / columns).ceil();
    (columns as usize, rows as usize)
}
