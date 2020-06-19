use std::collections::HashSet;

/// Returns a set of string slices that are anagrams of word.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    if possible_anagrams.is_empty() {
        return HashSet::new();
    }

    let sorted_chars = sort_chars(word);

    possible_anagrams
        .iter()
        .filter(|s| {
            s.len() == word.len()
                && s.to_lowercase() != word.to_lowercase()
                && sort_chars(s) == sorted_chars
        })
        .copied()
        .collect::<HashSet<&'a str>>()
}

/// Returns a lowercase string with the characters of word sorted alphabetically.
fn sort_chars(word: &str) -> String {
    let chars = &mut word.to_lowercase().chars().collect::<Vec<char>>()[..];

    chars.sort();

    chars.iter().collect::<String>()
}
