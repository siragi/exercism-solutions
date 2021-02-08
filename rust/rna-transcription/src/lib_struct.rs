/* Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement.
 */

#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // Construct new Dna from a string. If string contains invalid nucleotides return index of first invalid nucleotide;
        for (index, c) in dna.char_indices() {
            if !['G', 'C', 'T', 'A'].contains(&c) {
                return Err(index);
            };
        }

        Ok(Self {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        // Transform Dna into corresponding Rna

        let mut rna = String::new();
        for c in self.dna.chars() {
            match c {
                'G' => rna.push('C'),
                'C' => rna.push('G'),
                'T' => rna.push('A'),
                'A' => rna.push('U'),
                _ => unreachable!(),
            }
        }
        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // Construct new Rna from a string. If string contains invalid nucleotides return index of first invalid nucleotide;
        for (index, c) in rna.char_indices() {
            if !['G', 'C', 'U', 'A'].contains(&c) {
                return Err(index);
            };
        }

        Ok(Self {
            rna: rna.to_string(),
        })
    }
}
