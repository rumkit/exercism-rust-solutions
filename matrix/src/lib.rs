use std::str::FromStr;

pub struct Matrix {
    data: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data = input.lines()
            .flat_map(|line| line.split_whitespace())
            .map(|word| u32::from_str(word).unwrap())
            .collect::<Vec<u32>>();

        let rows = input.lines().count();
        let cols = data.len() / rows;

        Self { data, rows, cols }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if !(1..=self.rows).contains(&row_no) {
            return None;
        }

        Some(self.data.iter().copied().skip((row_no - 1) * self.cols).take(self.cols).collect())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if !(1..=self.cols).contains(&col_no) {
            return None;
        }

        Some(self.data.iter().copied().skip(col_no - 1).step_by(self.cols).collect())
    }
}
