use std::io; // import io library from the standard library

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(0..=100);

    // DEBUG
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();  // String is growable, UTF-8 text
                                            // :: means that new is an associated function of String type

    io::stdin()
        .read_line(&mut guess) // &mut --> mutable (mut) reference (&)
        .expect("Failed to read line"); // similar to expect statement in Python

    println!("You guessed: {guess}"); // {} --> placeholder containing a variable
}