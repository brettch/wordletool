use rayon::prelude::*;
use std::{cmp, cmp::Ordering, collections::HashSet};

use crate::bucket;

pub struct Guess<'a> {
    pub value: &'a Vec<char>,
    pub possible: bool,
    pub bucket_size_max: usize,
    pub bucket_variance: usize,
}

pub fn best_guesses<'a>(solution_words: &[&Vec<char>], guess_words: &'a [&Vec<char>]) -> Vec<Guess<'a>> {
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
            a.cmp(b)
        } else if a_is_possible {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut result: Vec<Guess> = prioritised_guess_words.par_iter().map(|&guess_word| {
        let bucket_map = bucket::bucket_guess(solution_words, guess_word);
        let (bucket_size_max, bucket_variance) = bucket_map.values()
            .map(|b| { b.len() })
            .fold((0, 1), |(max_bucket_size, bucket_variance), bucket_size| {
                (cmp::max(max_bucket_size, bucket_size), bucket_variance + (bucket_size * bucket_size))
            });

        Guess {
            value: guess_word,
            possible: solution_word_set.contains(guess_word),
            bucket_size_max,
            bucket_variance,
        }
    }).collect();

    // We sort by the following criteria:
    // 1. Bucket size max descending
    // 2. Bucket variance descending
    // 3. Possible solution word preferred
    // 4. Alphabetical
    result.sort_by(|a, b| {
        if a.bucket_size_max != b.bucket_size_max {
            return a.bucket_size_max.cmp(&b.bucket_size_max);
        }
        if a.bucket_variance != b.bucket_variance {
            return a.bucket_variance.cmp(&b.bucket_variance);
        }
        if a.possible != b.possible {
            return if a.possible { Ordering::Less } else { Ordering::Greater }
        }
        a.value.cmp(b.value)
    });

    result
}
