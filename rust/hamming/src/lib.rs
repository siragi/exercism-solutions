/// Return the Hamming distance between the strings (count of diffenerences),
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    Some(
        s1.chars()
            .zip(s2.chars())
            .filter(|(nucl1, nucl2)| nucl1 != nucl2)
            .count(),
    )
}
