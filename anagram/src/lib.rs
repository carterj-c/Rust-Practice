use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut candidates: HashSet<&'a str> = HashSet::new();
    let lower_base = word.to_lowercase();
    let base_word_cleaned = lower_base.trim(); // Cannot mutate this as this would persist throgh the loop
    let sorted_word: Vec<char> = sort_cleaned_word(base_word_cleaned);

    for candidate in possible_anagrams {

        let lower_base = candidate.to_lowercase();
        let cleaned_word: &str  = lower_base.trim();

        if cleaned_word == base_word_cleaned {
            continue;
        }
        // Now we sort the words
        let sorted_candidate: Vec<char> = sort_cleaned_word(cleaned_word);

        if sorted_candidate == sorted_word {
            candidates.insert(*candidate);
        }


    };

    candidates
}

fn sort_cleaned_word(cleaned: &str) -> Vec<char> {
    let mut char_vector: Vec<char> = cleaned.chars().collect();
    char_vector.sort();
    char_vector
}