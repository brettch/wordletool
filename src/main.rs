use std::io;

mod guess;
mod bucket;
mod hellowordl;
mod input;
mod matching;
mod solver;
mod words;

fn main() -> Result<(), io::Error> {
    println!("Which game do you wish to play?");
    println!("0: Wordle");
    println!("1: Hello Wordl");
    let game_index = input::get_index(0, 2)?;
    if game_index == 0 {
        let solution_words = words::solution_words();
        let all_words = words::all_words();
        println!("Let's play Wordle!");
        solver::interactive_solve(&solution_words.iter().collect::<Vec<_>>(), &all_words.iter().collect::<Vec<_>>())?;
    } else if game_index == 1 {
        println!("Let's play Hello Wordle!");
        println!("How many letters do you wish to use?");
        let word_length = input::get_index(4, 12)?;
        let solution_words: Vec<_> = hellowordl::words::solution_words();
        let all_words: Vec<_> = hellowordl::words::all_words();
        let solution_words: Vec<_> = solution_words
            .iter().filter(|&word| {
                word.len() == word_length
            })
            .collect();
        let all_words: Vec<_> = all_words
            .iter().filter(|&word| {
                word.len() == word_length
            })
            .collect();
        solver::interactive_solve(&solution_words, &all_words)?;
    }

    Ok(())
}
