use rand::Rng;
use std::cmp::Ordering;

pub fn play() {
    let secret_number = rand::rng().random_range(1..=100);

    println!("I've picked a number between 1 and 100.");
    println!("Can you guess it?");

    loop {
        let guess = read_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed it right.");
                break;
            }
        }
    }
}

fn read_guess() -> u32 {
    use std::io::{self, Write};

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
