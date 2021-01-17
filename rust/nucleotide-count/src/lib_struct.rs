use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;
    let nucleotides = "ACGT";
    if nucleotides.find(nucleotide) == None {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        if nucleotides.find(c) == None {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for n in "ACGT".chars() {
        counts.insert(n, count(n, dna)?);
    }
    Ok(counts)
}
