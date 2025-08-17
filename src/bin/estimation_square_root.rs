// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function estimates the square root of a number `n`.
fn estimate_square_root(n: f64) -> f64 {
    // First, find the integer part of the square root.
    // We can do this by taking the square root and truncating the decimal part.
    let integer_part = n.sqrt().trunc();

    // Find the two perfect squares the number `n` lies between.
    let lower_square = integer_part * integer_part;
    let upper_square = (integer_part + 1.0) * (integer_part + 1.0);

    // Calculate how far the number `n` is from the lower square.
    let distance_from_lower = n - lower_square;
    // Calculate the total distance between the two squares.
    let total_distance = upper_square - lower_square;

    // Estimate the decimal part of the square root.
    // This is done by finding the proportion of the distance `n` is from the lower square
    // relative to the total distance between the squares.
    let decimal_part_estimate = distance_from_lower / total_distance;

    // The final estimated square root is the integer part plus the estimated decimal part.
    integer_part + decimal_part_estimate
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter a number.
    println!("Enter a number to estimate its square root:");

    // Create a mutable string to store the user's input for the number.
    let mut number_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut number_input).expect("Failed to read line");

    // Parse the user's input into a 64-bit floating-point number.
    let number: f64 = match number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Prompt the user to enter the number of decimal places for the result.
    println!("Enter the number of decimal places for the estimate (or press Enter for the nearest whole number):");

    // Create a mutable string to store the user's input for the decimal places.
    let mut places_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut places_input).expect("Failed to read line");

    // Parse the user's choice into a 32-bit unsigned integer.
    // If the input is empty, default to 0 decimal places (rounding to the nearest whole number).
    let decimal_places: u32 = if places_input.trim().is_empty() {
        0
    } else {
        match places_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number of decimal places.");
                return;
            }
        }
    };

    // Calculate the estimated square root using the `estimate_square_root` function.
    let estimated_sqrt = estimate_square_root(number);

    // Round the estimated square root to the desired number of decimal places.
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    let rounded_estimate = (estimated_sqrt * multiplier).round() / multiplier;

    // Find the two perfect squares the number is between.
    let integer_part = number.sqrt().trunc() as u32;
    let lower_square = integer_part * integer_part;
    let upper_square = (integer_part + 1) * (integer_part + 1);

    // Print the explanation and the final result.
    println!("\n{} is between {} ({}²) and {} ({}²).", number, lower_square, integer_part, upper_square, integer_part + 1);
    println!("A sensible estimate for the square root of {} is ≈ {}", number, rounded_estimate);
}