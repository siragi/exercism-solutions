// Adapted version of kybernetikos's solution: https://exercism.io/tracks/rust/exercises/pascals-triangle/solutions/3f8a97463cc1430dbf90470a78ea0d9e
use std::iter::once;

pub struct PascalsTriangle {
    row_count: u32,
}

#[rustfmt::skip]
fn next_row(current_row: &Vec<u32>) -> Vec<u32> {
    once(1u32) // Let's start an Iterator with a 1
        .chain(
            current_row.iter() // and combine it with the input
                .zip(current_row.iter().skip(1)) // pair it with itself, but without the first element to tuples - ex. 1,2,1 with 2,1 -> (1,2),(2,1)
                .map(|(a, b)| a + b), // deconstruct the tuples to sum them up, (1,2),(2,1) -> 3,3
        )
        .chain(once(1u32))
        .collect::<Vec<u32>>()
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let first_row = vec![1u32];

        match self.row_count {
            0 => vec![],
            1 => vec![first_row],
            n => once(first_row.clone())
                .chain((1..n).scan(first_row, |current_row, _| {
                    // Begin with first_row as current row, '_' means that the range index n is irrelevant for the closure)
                    *current_row = next_row(current_row); // the "scan"-argument first_row is now serving as starting point for the next row, and since current_row passes on its state, we can start a "chain reaction".
                    Some(current_row.clone()) // unlike the somewhat similar fold() method, scan() requires its closure to return an Option.
                }))
                .collect::<Vec<Vec<u32>>>(),
        }
    }
}
