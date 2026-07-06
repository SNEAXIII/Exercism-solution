use std::collections::HashSet;
use itertools::Itertools;

fn sort_letters(word: &str) -> String {
    word.to_lowercase().chars().sorted().collect::<String>()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let sorted_word = sort_letters(word);
    let mut list =Vec::with_capacity(possible_anagrams.len());
    for candidate in possible_anagrams{
        if sorted_word == sort_letters(candidate) && word.to_lowercase() != *candidate.to_lowercase(){
            list.push(*candidate)
        }
    }
    HashSet::from_iter(list)
}
