use std::collections::HashSet;
use itertools::Itertools;

pub struct Word {
    word: String,
    repr: String,
}

impl Word {
    fn sort_letters(word: & str) -> String {
        word.to_lowercase().chars().sorted().collect::<String>()
    }
    pub fn new(word: &str) -> Self {
        Word{word:word.to_lowercase(),repr:Word::sort_letters(word)}
    }
}

impl PartialEq for Word{
    fn eq(&self,other:&Word) -> bool {
        self.repr == other.repr && self.word.to_lowercase() != other.word.to_lowercase()
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let word_word = Word::new(word);
    possible_anagrams.into_iter().filter(|candidate|word_word == Word::new(*candidate)).copied().collect()
}
