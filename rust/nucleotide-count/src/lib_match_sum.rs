// bucatini-coder's solution: https://exercism.io/tracks/rust/exercises/nucleotide-count/solutions/3c7df6a4f9ad47bf9f7ab30c1fd752c0
use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => dna
            .chars()
            .map(|c| match c {
                x if x == nucleotide => Ok(1usize),
                'A' | 'C' | 'G' | 'T' => Ok(0usize),
                _ => Err(c),
            })
            .sum(),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    ['A', 'C', 'G', 'T']
        .iter()
        .map(|&n| match count(n, dna) {
            Ok(v) => Ok((n, v)),
            Err(n) => Err(n),
        })
        .collect()
}
