/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut diffs: usize = 0;
    for (nucl1, nucl2) in s1.chars().zip(s2.chars()) {
        if nucl1 != nucl2 {
            diffs += 1;
        }
    }
    Some(diffs)
}
