use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {0}", secret_number);

    print!("Please input your guess: ");
    io::stdout()
        .flush()
        .expect("Failed to flush stdout!66");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess)
}
