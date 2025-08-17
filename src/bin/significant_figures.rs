// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function rounds a floating-point number `n` to a specified number of `sf` significant figures.
fn round_to_significant_figures(n: f64, sf: u32) -> f64 {
    // A number of significant figures of 0 is not meaningful, so we return 0.0.
    if sf == 0 {
        return 0.0;
    }

    // The core idea is to shift the decimal point so that the number of significant figures
    // corresponds to the number of digits before the decimal point, then round, and then shift back.

    // First, calculate the position of the most significant digit using the base-10 logarithm.
    // `n.abs().log10()` gives the power of 10 for the most significant digit.
    // `floor()` rounds this down to the nearest integer.
    // For example, for 52691, log10 is ~4.7, so the position is 4.
    // For 0.00097151, log10 is ~-3.01, so the position is -4.
    let position = n.abs().log10().floor();

    // Next, calculate the scaling factor (multiplier) to shift the decimal point.
    // This is 10 to the power of (significant figures - position - 1).
    // For 52691 to 1 sf: 10^(1 - 4 - 1) = 10^-4 = 0.0001
    // For 6.578 to 2 sf: 10^(2 - 0 - 1) = 10^1 = 10
    // For 0.00097151 to 3 sf: 10^(3 - (-4) - 1) = 10^6 = 1000000
    let multiplier = 10.0_f64.powi(sf as i32 - position as i32 - 1);

    // Now, apply the rounding logic:
    // 1. Multiply the number by the multiplier to shift the decimal point.
    //    52691 * 0.0001 = 5.2691
    //    6.578 * 10 = 65.78
    //    0.00097151 * 1000000 = 971.51
    // 2. Round the result to the nearest whole number.
    //    5.2691.round() = 5.0
    //    65.78.round() = 66.0
    //    971.51.round() = 972.0
    // 3. Divide the result by the multiplier to shift the decimal point back.
    //    5.0 / 0.0001 = 50000.0
    //    66.0 / 10 = 6.6
    //    972.0 / 1000000 = 0.000972
    let rounded = (n * multiplier).round() / multiplier;

    rounded
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
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Prompt the user to enter the number of significant figures.
    println!("Enter the number of significant figures:");

    // Create a mutable string to store the user's input for the significant figures.
    let mut sf_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut sf_input).expect("Failed to read line");

    // Parse the user's choice into a 32-bit unsigned integer.
    let significant_figures: u32 = match sf_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number of significant figures.");
            return;
        }
    };

    // Calculate the rounded number using the `round_to_significant_figures` function.
    let rounded_number = round_to_significant_figures(number, significant_figures);

    // Print the final result to the console.
    println!("The number rounded to {} significant figures is: {}", significant_figures, rounded_number);
}
