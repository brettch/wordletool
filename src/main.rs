use std::{env::args, process::exit};

mod bucket;
mod matching;
mod words;

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

    let solution_words = words::solution_words();
    let mut remaining_words = Vec::new();
    for word in solution_words.iter() {
        remaining_words.push(word);
    }
    let bucket_map = bucket::bucket_guess(&remaining_words, &test_word.chars().collect());

    let mut buckets: Vec<_> = bucket_map.iter().collect();
    buckets.sort_by(|a, b| {
        b.1.len().cmp(&a.1.len())
    });
    println!("Test word buckets");
    for bucket in buckets {
        println!("{} {:?}", bucket.1.len(), bucket.0.letters);
        println!("{:?}", bucket.1);
    }
}
