// Import the `io` (input/output) library from the standard library. 
// This library is used for reading from stdin and writing to stdout.
use std::io;

// Import the `Rng` trait from the `rand` crate. This trait defines methods for random number generation.
use rand::Rng;

// Import the `Ordering` enum from the `std::cmp` module, which is used for comparing values.
use std::cmp::Ordering;

// The entry point of the program.
fn main() {
    // Print a message to the console indicating the game has started.
    println!("Guess the number!");

    // Generate a secret number between 1 and 100 using rand's thread-local RNG.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Uncomment the line below to debug and see the secret number during development.
    // println!("The secret number is: {secret_number}");

    // Infinite loop to allow multiple guesses until the user guesses correctly.
    loop {
        println!("Please input your guess.");

        // Declare a mutable variable `guess` to store the user's guess, initializing it as an empty string.
        let mut guess = String::new();

        // Read a line from standard input and append it to `guess`. If reading fails, print an error and exit.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempt to convert the string guess to a u32. If conversion fails, prompt for another guess.
        // This uses pattern matching to handle the Result type returned by `parse()`.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Inform the user of their current guess.
        println!("You guessed: {guess}");

        // Compare the guess to the secret number, and print feedback.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop if the guess is correct.
            }
        }
    }
}
