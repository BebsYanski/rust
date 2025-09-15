use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!",);
    let mut rng = rand::rng();
    // includes 100
    let secret_number = rng.random_range(1..=100);
    let random: u8 = rng.random();
    //not inclusive
    let new_secret_number: u8 = rng.random_range(..100);

    // let new_secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {random} {secret_number}, {new_secret_number}",);

    // Add a loop
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        println!("{guess}");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Check if guess is a number and Convert to a number datatype
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            // since the result of parse is an enum = [ok,Err]. we can handle each arm (result) using match
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("You guessed: {guess}",);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
