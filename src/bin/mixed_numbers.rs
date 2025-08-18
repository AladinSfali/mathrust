// This program helps you convert mixed numbers to improper fractions.
// A mixed number is a whole number combined with a fraction, like 2 and 1/2.
// An improper fraction is a fraction where the numerator (top number) is bigger than or equal to the denominator (bottom number), like 5/2.

use std::io;

// This is the main function where our program starts.
fn main() {
    println!("Mixed Number to Improper Fraction Converter");
    println!("Enter the mixed number in the format whole_number numerator/denominator (e.g., 4 3/5)");

    // This loop allows the user to convert multiple mixed numbers without restarting the program.
    loop {
        // Create a new, empty String to hold the user's input.
        let mut input = String::new();

        // Read the user's input from the command line.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Remove any extra whitespace from the input.
        let clean_input = input.trim();

        // If the user types "exit", the program will stop.
        if clean_input == "exit" {
            break;
        }

        // Split the input into the whole number part and the fraction part.
        let parts: Vec<&str> = clean_input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid format. Please use the format whole_number numerator/denominator.");
            continue;
        }

        // Parse the whole number.
        let whole_number: Result<u32, _> = parts[0].parse();

        // Split the fraction part into numerator and denominator.
        let fraction_parts: Vec<&str> = parts[1].split('/').collect();
        if fraction_parts.len() != 2 {
            println!("Invalid fraction format. Please use numerator/denominator.");
            continue;
        }

        // Parse the numerator and denominator of the fractional part.
        let numerator: Result<u32, _> = fraction_parts[0].parse();
        let denominator: Result<u32, _> = fraction_parts[1].parse();

        // Check if all parts are valid numbers.
        if let (Ok(whole), Ok(num), Ok(den)) = (whole_number, numerator, denominator) {
            if den == 0 {
                println!("Error: Denominator cannot be zero.");
                continue;
            }

            println!("\nConverting mixed number {}({}/{}) to an improper fraction:", whole, num, den);

            // Step 1: Convert the whole number into a fraction with the same denominator.
            // To do this, multiply the whole number by the denominator.
            let equivalent_numerator_from_whole = whole * den;
            println!("1. Convert the whole number ({}) to a fraction with denominator {}.", whole, den);
            println!("   {} * {} = {}", whole, den, equivalent_numerator_from_whole);
            println!("   So, {} is equivalent to {}/{}", whole, equivalent_numerator_from_whole, den);

            // Step 2: Add the numerator of the original fraction to this new numerator.
            // The denominator remains the same.
            let improper_numerator = equivalent_numerator_from_whole + num;
            println!("2. Add the numerator of the original fraction ({}) to the new numerator ({})", num, equivalent_numerator_from_whole);
            println!("   {} + {} = {}", equivalent_numerator_from_whole, num, improper_numerator);

            println!("The improper fraction is: {}/{}", improper_numerator, den);
        } else {
            println!("Invalid input. Please enter valid numbers for the whole number, numerator, and denominator.");
        }
        println!("\nEnter another mixed number or type 'exit' to quit.");
    }
}
