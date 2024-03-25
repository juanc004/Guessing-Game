# Rust Guessing Game

Welcome to the Rust Guessing Game! This project is a hands-on introduction to Rust, covering fundamental concepts such as variables, user input handling, external crates, and control flow. The guessing game is a classic programming exercise, and building it in Rust offers a unique opportunity to explore the language's features and idioms.

## About the Game

The guessing game challenges the player to guess a random number generated by the program. Here's how it works:

- The program generates a random integer between 1 and 100.
- It prompts the player to enter a guess.
- Upon receiving input, the program evaluates the guess:
  - If the guess is too low, it prompts the player to guess higher.
  - If the guess is too high, it suggests guessing lower.
  - If the guess is correct, the program congratulates the player and exits.

This simple game introduces several Rust concepts, including `let` bindings, the `match` control flow operator, methods, associated functions, and the use of external crates (in this case, for generating random numbers).

## Getting Started

To play the guessing game, you'll need to have Rust installed on your machine. If you haven't already, you can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, follow these steps to download and run the game:

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-guessing-game.git
   ```

2. Change into the project directory:
```
cd rust-guessing-game
```

3. Compile and run the game:
```
cargo run
```

## Concepts Covered:

- Standard Input and Output (I/O): Demonstrates basic I/O operations in Rust using std::io, including reading from stdin and writing to stdout. It highlights the importance of handling user input in interactive applications.

- Mutable Variables: Shows how to declare mutable variables in Rust with let mut, allowing their values to be changed after initialization, which is essential for capturing and updating user input.

- String Handling: Introduces string creation and manipulation, emphasizing the dynamic, growable nature of String objects in Rust and their ability to be modified.

- Type Conversion and Error Handling: Utilizes Rust's powerful pattern matching with match expressions to safely convert strings to integers and handle potential errors gracefully, showcasing Rust's approach to error handling that favors explicitness and safety.

- Loops: Utilizes loop, an infinite loop construct, to continuously prompt the user for input until a specific condition is met (i.e., the correct guess), demonstrating control flow mechanisms in Rust.

- Random Number Generation: Leverages the rand crate to generate random numbers, illustrating how to use external crates and their functionalities, as well as the trait system in Rust through the use of the Rng trait for random number generation.

- Enums and Pattern Matching: Introduces the use of the Ordering enum and pattern matching to compare numbers, highlighting Rust's enum and exhaustive pattern matching capabilities for writing clear and concise decision-making logic.

- Break Statement: Shows the use of break to exit loops, a common control flow mechanism that allows terminating loop execution based on a condition, in this case, guessing the correct number.

