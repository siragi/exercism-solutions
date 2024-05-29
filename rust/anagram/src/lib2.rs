use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();

    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();

    possible_anagrams
        .iter()
        .copied()
        .filter(|&w| {
            let lower = w.to_lowercase();
            lower != lower_word && {
                let mut l: Vec<char> = lower.chars().collect();
                l.sort_unstable();
                l == sorted
            }
        })
        .collect()
}
