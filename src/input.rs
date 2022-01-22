use std::io;

pub fn get_index(upper_bound: usize) -> Result<usize, io::Error> {
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

fn get_user_input() -> Result<String, io::Error> {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    let result = user_input.trim_end().to_string();
    println!("User input: {}", result);
    Ok(result)
}
