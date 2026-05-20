pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        if self.row_count >= 1 {
            rows.push(vec![1]);
        }
        if self.row_count >= 2 {
            rows.push(vec![1,1]);
        }

        for count in 3..=self.row_count {
            let previous_row = &rows[(count - 2) as usize];
            let mut new_row = vec![1];
            for i in 1..previous_row.len() {
                new_row.push(previous_row[i - 1] + previous_row[i]);
            }
            new_row.push(1);
            rows.push(new_row);
        }

        rows
    }
}
