use std::io;
fn main() {
    println!("Guessing Game!");
    println!("You have 5 tries to guess a number between 1 and 50.");
    println!("After each guess you will be told if the number is higher or lower than your guess.");
    let mut min: u8 = 1;
    let mut max: u8 = 50;
    let mut middle: u8 = 25;

    for guess_count in (0..5).rev() {
        let guess = get_guess(min, max);
        if guess >= middle {
            println!("Too high!");
            max = guess;
            middle = (min + max) / 2;
        } else {
            println!("Too low!");
            min = guess;
            middle = (min + max) / 2;
        }
        match guess_count{
            0 => println!("The number was: {middle}"),
            1 => println!("1 guess remaining"),
            _ => println!("{guess_count} guesses remaining"),
        }
    }
}

fn get_guess(min: u8, max: u8) -> u8 {
    println!("Enter a number between {min} and {max}:");
    let mut input = String::new();
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Couldn't read input. Please try again.");
                continue;
            }
        }

        match input.trim().parse::<u8>() {
            Ok(n) => {
                if n < min || n > max {
                    println!("Invalid input: number must be between {min} and {max}.");
                    continue;
                }
                return n;
            }
            Err(_) => {
                println!("Invalid input: not a valid positive 8-bit integer.");
                continue;
            }
        }
    }
}
