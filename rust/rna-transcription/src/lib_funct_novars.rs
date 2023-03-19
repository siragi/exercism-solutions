/* Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement.
 */

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // Construct new Dna from a string. If string contains invalid nucleotides return index of first invalid nucleotide;
        Ok(Dna(dna
            .char_indices()
            .map(|(i, c)| match c {
                'A' | 'T' | 'C' | 'G' => Ok(c),
                _ => return Err(i),
            })
            .collect::<Result<String, usize>>()?))
    }

    pub fn into_rna(self) -> Rna {
        // Transform Dna into corresponding Rna
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => unreachable!(), // will never be hit -
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // Construct new Rna from a string. If string contains invalid nucleotides return index of first invalid nucleotide;
        Ok(Rna(rna
            .char_indices()
            .map(|(i, c)| match c {
                'A' | 'U' | 'C' | 'G' => Ok(c),
                _ => return Err(i),
            })
            .collect::<Result<String, usize>>()?))
    }
}
