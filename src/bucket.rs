use std::collections::HashMap;

use crate::matching::{WordMatch, match_chars};

pub fn bucket_guess<'a>(solution_set: &'a [&Vec<char>], guess: &[char]) -> HashMap<WordMatch, Vec<&'a Vec<char>>> {
    let mut bucket_map: HashMap<WordMatch, Vec<&Vec<char>>> = HashMap::new();

    for &word in solution_set {
        let word_match = match_chars(guess, word);

        match bucket_map.get_mut(&word_match) {
            Some(bucket) => {
                bucket.push(word);
            },
            _ => {
                bucket_map.insert(word_match, vec![word]);
            },
        };
    }

    bucket_map
}
