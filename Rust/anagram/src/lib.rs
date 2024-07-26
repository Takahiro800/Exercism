use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_sorted = get_sorted(&word);

    possible_anagrams
        .iter()
        .filter(|canditate| {
            let canditate = canditate.to_lowercase();

            word != canditate && word_sorted == get_sorted(&canditate)
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();

    word_sorted.sort_unstable();
    word_sorted
}

