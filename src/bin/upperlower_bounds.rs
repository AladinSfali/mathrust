// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function calculates the lower and upper bounds of a number `n` that has been rounded
// to a certain `degree_of_accuracy`.
fn calculate_bounds(n: f64, degree_of_accuracy: f64) -> (f64, f64) {
    // The error margin is half of the degree of accuracy.
    // For example, if a number is rounded to the nearest 10, the degree of accuracy is 10,
    // and the error margin is 10 / 2 = 5.
    let error_margin = degree_of_accuracy / 2.0;

    // The lower bound is the number minus the error margin.
    let lower_bound = n - error_margin;
    // The upper bound is the number plus the error margin.
    let upper_bound = n + error_margin;

    (lower_bound, upper_bound)
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter the rounded number.
    println!("Enter the rounded number:");

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

    // Prompt the user to enter the degree of accuracy.
    println!("Enter the degree of accuracy (e.g., 1 for nearest whole number, 10 for nearest ten, 0.1 for one decimal place):");

    // Create a mutable string to store the user's input for the degree of accuracy.
    let mut accuracy_input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut accuracy_input).expect("Failed to read line");

    // Parse the user's input into a 64-bit floating-point number.
    let degree_of_accuracy: f64 = match accuracy_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid degree of accuracy.");
            return;
        }
    };

    // Calculate the lower and upper bounds using the `calculate_bounds` function.
    let (lower_bound, upper_bound) = calculate_bounds(number, degree_of_accuracy);

    // Print the final result to the console.
    println!("The lower bound is: {}", lower_bound);
    println!("The upper bound is: {}", upper_bound);
}
