use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Wait for input
        print!("Please input your guess (`quit` or `exit` to exit): ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout!");

        // Get input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Try to parse given string to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // With `quit` or `exit`, break the loop -> exit, any other continues to next round
                match guess.to_lowercase().trim() {
                    "quit" | "exit" => {
                        println!("Received {}, exiting program...", guess.trim());
                        break;
                    },
                    _ => {
                        println!("Not a valid guess: {}. Continuing...", guess.trim());
                        continue;
                    }
                }
            },
        };

        // Guessed value and was it less, greater or equal. When equal -> exit
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! You win!");
                break;
            },
        }
    }
}
