use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new(); // how many times to play; get from user

    println!("How many times do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading how many times");

    let num_guesses: u8 = how_many.trim().parse().expect("Not a number");

    let correct = rand::rng().random_range(1..=10);
    println!("Hello, world!");

    // println!("correct: {correct}");
    println!("Hey, guess a number 1-10:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading guess");

        // error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        // Another method
        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed CORRECT");
                break;
            }
        };
    }
}
