use std::{env::args, process::exit};

#[derive(Debug, PartialEq, Eq, Clone)]
enum LetterMatch {
    None,
    Partial,
    Exact
}

#[derive(Debug, PartialEq, Eq)]
struct WordMatch {
    letters: Vec<LetterMatch>,
}

fn match_words(test_word: &str, target_word: &str) -> WordMatch {
    let test_chars: Vec<_> = test_word.chars().collect();
    let target_chars: Vec<_> = target_word.chars().collect();

    match_chars(test_chars, target_chars)
}

fn match_chars(test_chars: Vec<char>, target_chars: Vec<char>) -> WordMatch {
    assert_eq!(test_chars.len(), target_chars.len());

    let word_length = test_chars.len();
        let mut letter_matches: Vec<LetterMatch> = vec![LetterMatch::None; word_length];
    let mut letters_available: Vec<bool> = vec![true; word_length];

    // Check for exact matches.
    for i in 0..word_length {
        if test_chars[i] == target_chars[i] {
            letter_matches[i] = LetterMatch::Exact;
            letters_available[i] = false;
        }
    }

    // Check for partial matches.
    for test_i in 0..word_length {
        // Only process unmatched letters, some may have been exact matches.
        if letter_matches[test_i] == LetterMatch::None {
            // See if there is a remaining letter in a different position for a partial match.
            for target_i in 0..word_length {
                if letters_available[target_i] && test_chars[test_i] == target_chars[target_i] {
                    letter_matches[test_i] = LetterMatch::Partial;
                    letters_available[target_i] = false;
                    break;
                }
            }
        }
    }
    
    WordMatch {
        letters: letter_matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_match() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("abcde", "fghij"),
        );
    }

    #[test]
    fn test_full_exact_match() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Exact,
                LetterMatch::Exact,
                LetterMatch::Exact,
                LetterMatch::Exact,
                LetterMatch::Exact,
            ]},
            match_words("abcde", "abcde"),
        );
    }

    #[test]
    fn test_one_exact_match() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Exact,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("abcde", "aghij"),
        );
    }

    #[test]
    fn test_one_partial_match() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Partial,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("abcde", "fghia"),
        );
    }

    #[test]
    fn test_one_partial_match_with_two_possible() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Partial,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("abcde", "fghaa"),
        );
    }

    #[test]
    fn test_two_partial_match_with_one_possible() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Partial,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("aacde", "fghia"),
        );
    }

    #[test]
    fn test_one_partial_one_exact_with_same_letter() {
        assert_eq!(
            WordMatch { letters: vec![
                LetterMatch::Partial,
                LetterMatch::Exact,
                LetterMatch::None,
                LetterMatch::None,
                LetterMatch::None,
            ]},
            match_words("aacde", "fahia"),
        );
    }
}

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
    println!("Match: {:?}", match_words(test_word, target_word).letters);
}
