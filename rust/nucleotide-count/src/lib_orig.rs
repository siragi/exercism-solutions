use std::collections::HashMap;
use std::iter::once;

// 'A' for adenine, 'C' for cytosine, 'G' for guanine, and 'T' for thymine.
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match dna
        .chars()
        .chain(once(nucleotide))
        .find(|c| !['A', 'C', 'G', 'T'].contains(c))  // finds first non nucleotide
    {
        Some(non_nucleotide) => {
            Err(non_nucleotide)
        },
        None => { // dna and the nucleotide to look for are valid nucleotides
            Ok(dna.chars().fold(0 as usize, |counter, c| -> usize {
                if c == nucleotide {
                    counter + 1
                } else {
                    counter
                }
            }))
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    //     "How much of every nucleotide type is contained inside DNA string?",

    Ok("ACGT"
        .chars()
        .zip(
            "ACGT"
                .chars()
                .map(|c| count(c, dna))
                .collect::<Result<Vec<usize>, char>>()?,
        )
        .collect())
}
