use crate::bucket;

pub fn best_guesses<'a>(solution_set: &Vec<&Vec<char>>, guess_set: &'a Vec<&Vec<char>>) -> Vec<&'a Vec<char>> {
    assert_ne!(0, solution_set.len());
    assert_ne!(0, guess_set.len());

    let mut result = Vec::new();
    let mut best_max_bucket_size = usize::MAX;
    for &current_guess in guess_set {
        let bucket_map = bucket::bucket_guess(solution_set, current_guess);
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
