use std::{env::args, process::exit};

mod matching;

fn main() {
    let args: Vec<_> = args().collect();
    println!("args: {:?}", args);
    if args.len() != 3 {
        println!("Usage: wordletool <test word> <target word>");
        exit(-1);
    }
    let test_word = &args[1];
    let target_word = &args[2];
    println!("Test word: {}", test_word);
    println!("Target word: {}", target_word);
    println!(
        "Match: {:?}",
        matching::match_words(test_word, target_word).letters
    );
}
