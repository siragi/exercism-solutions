/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let string2 = Vec::from(s2);
    if s1.len() != s2.len() {
        return None;
    }
    let mut diffs: usize = 0;
    for (i, nucleotide) in s1.bytes().enumerate() {
        if nucleotide != string2[i] {
            diffs += 1;
        }
    }
    Some(diffs)
}
