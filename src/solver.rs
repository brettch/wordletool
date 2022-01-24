use std::{io, collections::HashSet};

use crate::{bucket, guess, matching};
use crate::input;

pub fn interactive_solve(solution_words: &[&Vec<char>], guess_words: &[&Vec<char>]) -> Result<(), io::Error> {
    let mut current_words: Vec<_> = solution_words.iter().copied().collect();

    while current_words.len() > 1 {
        println!("{} words are available", current_words.len());
        println!("Calculating best guesses ...");
        // let guess_options = guess::best_guesses(&current_words, guess_words);
        let guess_options = guess::best_guesses(&current_words, guess_words);
        display_guess_options(&guess_options);

        println!("Which guess do you pick (enter number):");
        let guess_index = input::get_index(0, guess_options.len())?;
        let guess = &guess_options[guess_index];
        println!("Guess selected: {}", chars_to_string(guess.value));

        let bucket_map = bucket::bucket_guess(&current_words, guess.value);
        let mut match_options: Vec<_> = bucket_map.keys().collect();
        match_options.sort();
        display_match_options(&match_options);

        println!("Which match value did you get (enter number):");
        let match_index = input::get_index(0, match_options.len())?;
        let word_match = match_options[match_index];
        println!("Match selected: {:?}", word_match);

        println!("Calculating remaining words ...");
        let bucket = bucket_map.get(word_match).unwrap();
        // This may seem unusual but we can't consume the bucket directly because it runs into borrow checker issues,
        // we must consume from the original current_words collection instead.
        let bucket_set: HashSet<_> = bucket.iter().copied().collect();
        current_words = current_words.iter()
            .copied()
            .filter(|word| {
                bucket_set.contains(word)
            }).collect();
    }

    println!("Solution is: {}", chars_to_string(current_words[0]));

    Result::Ok(())
}

fn display_guess_options(guess_options: &[guess::Guess]) {
    const MAXIMUM_TO_DISPLAY: usize = 20;

    println!("Guess Options (low max bucket size and low variance are better)");

    for (i, guess) in guess_options.iter().take(MAXIMUM_TO_DISPLAY).enumerate() {
        println!(
            "{}: {:?} (bucket_size_max={}, bucket_variance={}, possible={})",
            i,
            chars_to_string(guess.value),
            guess.bucket_size_max,
            guess.bucket_variance,
            guess.possible,
        );
    }
}

fn display_match_options(match_options: &[&matching::WordMatch]) {
    println!("Match Options");
    for (i, &option) in match_options.iter().enumerate() {
        println!("{}: {:?}", i, option.letters);
    }
}

fn chars_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}
