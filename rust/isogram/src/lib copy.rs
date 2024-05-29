pub fn check(candidate: &str) -> bool {
    for (i, char) in candidate.to_ascii_lowercase().chars().enumerate() {
        if !char.is_ascii_alphabetic() {
            continue;
        }
        for next_char in candidate[(i + 1)..].chars() {
            if char == next_char {
                return false;
            }
        }
    }
    true
}
