pub fn abbreviate(phrase: &str) -> String {
    // Given a phrase, return its acronym
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            match word {
                "" => vec![],
                w if w.to_uppercase() == w => vec![w.chars().nth(0).unwrap()], // for 'WORDS'
                w if w.to_lowercase() == w => vec![w.chars().nth(0).unwrap().to_ascii_uppercase()], // for 'words'
                w => w.chars().filter(|c| c.is_uppercase()).collect(), // for 'wOrDs' -> 'OD' or 'CamelCase' -> 'CC'
            }
        })
        .collect()
}
