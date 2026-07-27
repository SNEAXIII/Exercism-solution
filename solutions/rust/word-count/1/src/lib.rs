use std::collections::HashMap;

fn extract_and_push(i_start: usize, i_end: usize, words: &str, results: &mut HashMap<String, u32>) {
    let sanitized_word = words[i_start..i_end].trim_matches('\'');
    if sanitized_word.is_empty() {
        return;
    }
    *results.entry(sanitized_word.to_lowercase()).or_insert(0) += 1;
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut results = HashMap::new();
    let mut is_word = false;
    let mut first_index = 0;
    for (index, _char) in words.chars().enumerate() {
        if _char.is_ascii_alphanumeric() || _char == '\'' {
            if !is_word {
                first_index = index;
            }
            is_word = true
        } else {
            if is_word {
                extract_and_push(first_index, index, words, &mut results);
            }
            is_word = false;
        }
    }
    if is_word {
        extract_and_push(first_index, words.len(), words, &mut results);
    }
    results
}
