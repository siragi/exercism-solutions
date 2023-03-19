/* Adaptation of menski's solution: https://exercism.io/tracks/rust/exercises/acronym/solutions/ae9c39fa75f145b0a1a12b92963c18d7
 */
pub fn abbreviate(name: &str) -> String {
    // Given a phrase, return its acronym
    name.split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                // Example: take first letter of a "wORD" or "woRd"-> 'w'
                word.chars() // Begins again! at first letter of "wORD" or "woRd"
                    .skip_while(|c| c.is_uppercase()) // skipwhile stops skipping the rest at the first lowercase (takes all beginning with lowercase)
                    .filter(|c| c.is_uppercase()), // 'O','R' and 'D' vere skipped before and can no more be taken (filtered) here. in the second case "oRd" was not skipped and 'R' is filtered out.
            )
        })
        .collect::<String>()
        .to_uppercase() // Example creates "W" or "WR" respectively...
}
