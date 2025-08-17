// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function rounds a floating-point number `n` to a specified number of `decimal_places`.
fn round_to_decimal_places(n: f64, decimal_places: u32) -> f64 {
    // To round to a specific number of decimal places, we can use a multiplier.
    // The multiplier is calculated as 10 to the power of the number of decimal places.
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    // The rounding logic is as follows:
    // 1. Multiply the number by the multiplier: 8.9471 * 100.0 = 894.71 (for 2 decimal places)
    // 2. Round the result to the nearest whole number: 894.71.round() = 895.0
    // 3. Divide the result by the multiplier: 895.0 / 100.0 = 8.95
    (n * multiplier).round() / multiplier
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

    // Prompt the user to enter the number of decimal places.
    println!("Enter the number of decimal places to round to:");

    // Create a mutable string to store the user's input for the decimal places.
    let mut places_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut places_input).expect("Failed to read line");

    // Parse the user's choice into a 32-bit unsigned integer.
    let decimal_places: u32 = match places_input.trim().parse() {
        // If parsing is successful, assign the value to the `decimal_places` variable.
        Ok(num) => num,
        // If parsing fails, print an error message and exit the program.
        Err(_) => {
            println!("Invalid number of decimal places.");
            return;
        }
    };

    // Calculate the rounded number using the `round_to_decimal_places` function.
    let rounded_number = round_to_decimal_places(number, decimal_places);

    // Print the final result to the console.
    println!("The rounded number is: {}", rounded_number);
}
