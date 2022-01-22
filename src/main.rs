use std::io;

mod guess;
mod bucket;
mod input;
mod matching;
mod solver;
mod words;

fn main() -> Result<(), io::Error> {
    let solution_words = words::solution_words();
    let all_words = words::all_words();

    solver::interactive_solve(&solution_words.iter().collect::<Vec<_>>(), &all_words.iter().collect::<Vec<_>>())?;

    Ok(())
}
