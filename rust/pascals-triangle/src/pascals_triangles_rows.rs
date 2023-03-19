pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // create Pascal's triangle with {} rows", row_count)
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        let mut row: Vec<u32> = vec![];

        for _ in 0..row_count {
            match rows.last() {
                Some(last_row) => {
                    let mut sum_windows: Vec<u32>;
                    if last_row.len() >= 2 {
                        sum_windows = last_row
                            .windows(2)
                            .map(|window| window.iter().sum())
                            .collect();
                    } else {
                        sum_windows = vec![];
                    }
                    row.push(1);
                    row.append(&mut sum_windows);
                    row.push(1);
                }
                None => row = vec![1],
            }
            rows.push(row);
            row = vec![];
        }
        Self { rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
