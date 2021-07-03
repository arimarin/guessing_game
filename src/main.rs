use std::io::{self, Write};
use std::cmp::Ordering;
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

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct! You win!"),
    }
}
