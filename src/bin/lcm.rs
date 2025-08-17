// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.

// This function calculates the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
fn gcd(a: u32, b: u32) -> u32 {
    // The Euclidean algorithm is an efficient method for computing the GCD of two integers.
    // It is based on the principle that the greatest common divisor of two numbers does not change
    // if the larger number is replaced by its difference with the smaller number.
    let mut temp_a = a;
    let mut temp_b = b;
    while temp_b != 0 {
        let t = temp_b;
        temp_b = temp_a % temp_b;
        temp_a = t;
    }
    temp_a
}

// This function calculates the least common multiple (LCM) of two numbers.
fn lcm(a: u32, b: u32) -> u32 {
    // The LCM of two numbers can be calculated using the formula: LCM(a, b) = (|a * b|) / GCD(a, b).
    // To avoid potential overflow from `a * b`, we can rewrite the formula as: LCM(a, b) = |a / GCD(a, b) * b|.
    // If either `a` or `b` is zero, the LCM is 0.
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter a list of numbers separated by spaces.
    println!("Enter a list of numbers separated by spaces to find their LCM:");

    // Create a mutable string to store the user's input.
    let mut input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the user's input into a vector of 32-bit unsigned integers.
    let numbers: Vec<u32> = input
        .split_whitespace() // Split the input string by whitespace.
        .filter_map(|s| s.parse().ok()) // Attempt to parse each part into a u32, filtering out any that fail.
        .collect(); // Collect the valid numbers into a vector.

    // Check if any numbers were entered.
    if numbers.is_empty() {
        println!("No valid numbers were entered.");
        return;
    }

    // Calculate the LCM of the list of numbers.
    // `fold` is used to apply the `lcm` function cumulatively to the items of the vector.
    // It starts with an initial value of 1 (the identity for LCM) and applies the `lcm` function
    // to the current accumulator and each number in the vector.
    let result = numbers.iter().fold(1, |acc, &num| lcm(acc, num));

    // Print the final result to the console.
    println!("The least common multiple is: {}", result);
}
