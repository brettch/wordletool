use std::io;

use crate::{bucket, guess, matching};

pub fn interactive_solve(solution_words: &Vec<&Vec<char>>, guess_words: &Vec<&Vec<char>>) -> Result<(), io::Error> {
    let mut current_words: Vec<_> = solution_words.iter().map(|&f| { f }).collect();

    while current_words.len() > 1 {
        println!("{} words are available", current_words.len());
        println!("Calculating best guesses ...");
        // let guess_options = guess::best_guesses(&current_words, guess_words);
        let guess_options = guess::best_guesses(&current_words, &guess_words);
        display_guess_options(&guess_options, &current_words);

        println!("Which guess do you pick (enter number):");
        let guess_index = get_user_input_index(guess_options.len())?;
        let guess = guess_options[guess_index];
        println!("Guess selected: {}", chars_to_string(guess));

        let bucket_map = bucket::bucket_guess(&current_words, guess);
        let mut match_options: Vec<_> = bucket_map.keys().collect();
        match_options.sort();
        display_match_options(&match_options);

        println!("Which match value did you get (enter number):");
        let match_index = get_user_input_index(match_options.len())?;
        let word_match = match_options[match_index];
        println!("Match selected: {:?}", word_match);

        println!("Calculating remaining words ...");
        let bucket = bucket_map.get(word_match).unwrap();
        // This is very inefficient, can't consume the bucket directly because it runs into borrow checker issues.
        current_words = current_words.iter()
            .map(|&word| word)
            .filter(|word| {
                bucket.contains(word)
            }).collect();
    }

    println!("Solution is: {}", chars_to_string(current_words[0]));

    Result::Ok(())
}

fn display_guess_options(guess_options: &Vec<&Vec<char>>, remaining_words: &Vec<&Vec<char>>) {
    println!("Guess Options");
    for (i, &guess) in guess_options.iter().enumerate() {
        let in_remaining_words = remaining_words.contains(&guess);
        println!("{}: {:?} {}", i, chars_to_string(guess), if in_remaining_words { "possible" } else { "not possible" } );
    }
}

fn display_match_options(match_options: &Vec<&matching::WordMatch>) {
    println!("Match Options");
    for (i, &option) in match_options.iter().enumerate() {
        println!("{}: {:?}", i, option.letters);
    }
}

fn get_user_input() -> Result<String, io::Error> {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    let result = user_input.trim_end().to_string();
    println!("User input: {}: ", result);
    Ok(result)
}

fn get_user_input_index(upper_bound: usize) -> Result<usize, io::Error> {
    loop {
        print!("<0..{}>: ", upper_bound - 1);
        let user_input = get_user_input()?;
        let index = match user_input.parse::<usize>() {
            Ok(number)  => number,
            Err(_) => {
                println!("Invalid number, try again ...");
                continue;
            },
        };
        if index >= upper_bound {
            println!("Number is too high, please try again ...");
            continue;
        }
        return Ok(index);
    }
}

fn chars_to_string(chars: &Vec<char>) -> String {
    chars.iter().collect()
}
