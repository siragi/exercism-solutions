pub fn check(candidate: &str) -> bool {
    candidate
        .to_ascii_lowercase()
        .chars()
        .enumerate()
        .all(|(i, char)| {
            !char.is_ascii_alphabetic()
                || candidate[(i + 1)..]
                    .chars()
                    .all(|next_char| char != next_char)
        })
}
