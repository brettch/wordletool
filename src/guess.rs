use std::{cmp::Ordering, collections::HashSet};

use crate::bucket;

pub fn best_guesses<'a>(solution_words: &Vec<&Vec<char>>, guess_words: &'a Vec<&Vec<char>>) -> Vec<&'a Vec<char>> {
    assert_ne!(0, solution_words.len());
    assert_ne!(0, guess_words.len());

    // We prioritise (i.e. sort) by two criteria:
    // 1. Guesses in the possible solution are preferred.
    // 2. Alphabetically.
    let solution_word_set: HashSet<_> = solution_words.iter().cloned().collect();
    let mut prioritised_guess_words = guess_words.to_vec();
    prioritised_guess_words.sort_by(|a, b| {
        let a_is_possible = solution_word_set.contains(a);
        let b_is_possible = solution_word_set.contains(b);

        if a_is_possible == b_is_possible {
            return a.cmp(b);
        }
        if a_is_possible {
            return Ordering::Less;
        }
        return Ordering::Greater;
    });

    let mut result = Vec::new();
    let mut best_max_bucket_size = usize::MAX;
    for current_guess in prioritised_guess_words {
        let bucket_map = bucket::bucket_guess(solution_words, current_guess);
        let max_bucket_size = bucket_map.values()
            .map(|b| { b.len() })
            .max()
            .unwrap_or(usize::MAX);
        if max_bucket_size < best_max_bucket_size {
            best_max_bucket_size = max_bucket_size;
            result.clear();
        }
        if max_bucket_size <= best_max_bucket_size {
            result.push(current_guess);
        }
    }

    result
}
