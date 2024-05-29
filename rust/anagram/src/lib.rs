use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = lower_word.chars().collect();
    sorted_word.sort_unstable();
    possible_anagrams
        .iter()
        .copied()
        // .map(|&anagram| anagram) // copied() does the same. Done because collect can not use Items of &&str
        .filter(|&anagram| {
            let lower_anagram = anagram.to_lowercase();
            let mut sorted_anagram: Vec<char> = lower_anagram.chars().collect();
            sorted_anagram.sort_unstable();
            // stop is no anagram of stop, but post would be.
            sorted_anagram == sorted_word && lower_anagram != lower_word
        })
        .collect()
}
