use std::collections::HashMap;

use crate::matching::{WordMatch, match_chars};

pub fn bucket_guess<'a>(solution_set: &'a Vec<&Vec<char>>, guess: &Vec<char>) -> HashMap<WordMatch, Vec<&'a Vec<char>>> {
    let mut bucket_map: HashMap<WordMatch, Vec<&Vec<char>>> = HashMap::new();

    for &word in solution_set {
        let word_match = match_chars(guess, word);

        match bucket_map.get_mut(&word_match) {
            Some(bucket) => {
                bucket.push(word);
            },
            _ => {
                let mut bucket: Vec<&Vec<char>> = Vec::new();
                bucket.push(word);
                bucket_map.insert(word_match, bucket);
            },
        };
    }

    bucket_map
}
