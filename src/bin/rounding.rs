// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function rounds a floating-point number `n` to the specified `place`.
// The `place` determines the position to round to (e.g., 1.0 for whole number, 10.0 for ten, etc.).
fn round_to_place(n: f64, place: f64) -> f64 {
    // The rounding logic is based on the formula: round(n / place) * place.
    // For example, to round 5468.9 to the nearest ten (place = 10.0):
    // 1. Divide by the place: 5468.9 / 10.0 = 546.89
    // 2. Round to the nearest whole number: 546.89.round() = 547.0
    // 3. Multiply by the place: 547.0 * 10.0 = 5470.0
    (n / place).round() * place
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter a number.
    println!("Enter a number to round:");

    // Create a mutable string to store the user's input for the number.
    let mut number_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut number_input).expect("Failed to read line");

    // Parse the user's input into a 64-bit floating-point number.
    let number: f64 = match number_input.trim().parse() {
        // If parsing is successful, assign the number to the `number` variable.
        Ok(num) => num,
        // If parsing fails, print an error message and exit the program.
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Prompt the user to choose the rounding place.
    println!("Choose what to round to:");
    println!("1: Nearest whole number");
    println!("2: Nearest ten");
    println!("3: Nearest hundred");
    println!("4: Nearest thousand");

    // Create a mutable string to store the user's input for the choice.
    let mut choice_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut choice_input).expect("Failed to read line");

    // Parse the user's choice into a 32-bit unsigned integer.
    let choice: u32 = match choice_input.trim().parse() {
        // If parsing is successful, assign the choice to the `choice` variable.
        Ok(num) => num,
        // If parsing fails, print an error message and exit the program.
        Err(_) => {
            println!("Invalid choice.");
            return;
        }
    };

    // Determine the rounding place based on the user's choice.
    let place = match choice {
        1 => 1.0,      // Nearest whole number
        2 => 10.0,     // Nearest ten
        3 => 100.0,    // Nearest hundred
        4 => 1000.0,   // Nearest thousand
        // If the choice is invalid, print an error message and exit.
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    // Calculate the rounded number using the `round_to_place` function.
    let rounded_number = round_to_place(number, place);

    // Print the result with an approximation sign.
    println!("{} ≈ {}", number, rounded_number);
}
