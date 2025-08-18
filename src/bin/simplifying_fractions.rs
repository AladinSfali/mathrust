// This program helps you simplify fractions to their lowest terms.
// Simplifying a fraction means writing an equivalent fraction using the smallest possible numbers.
// This is also known as expressing a fraction in its lowest terms.

use std::io;

// This is the main function where our program starts.
fn main() {
    println!("Fraction Simplifier");
    println!("Enter the fraction in the format numerator/denominator (e.g., 24/30)");

    // This loop allows the user to simplify multiple fractions without restarting the program.
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

        // Split the input string into numerator and denominator at the "/" sign.
        let parts: Vec<&str> = clean_input.split('/').collect();
        if parts.len() != 2 {
            println!("Invalid format. Please use the format numerator/denominator.");
            continue;
        }

        // Parse the numerator and denominator into numbers.
        let numerator: Result<u32, _> = parts[0].parse();
        let denominator: Result<u32, _> = parts[1].parse();

        // Check if both parts are valid numbers.
        if let (Ok(mut num), Ok(mut den)) = (numerator, denominator) {
            if den == 0 {
                println!("Error: Denominator cannot be zero.");
                continue;
            }

            println!("\nExpressing {}/{} in its lowest terms:", num, den);

            // We will repeatedly divide by common factors until no more common factors exist.
            // The greatest common divisor (GCD) is the largest number that divides two or more numbers without leaving a remainder.
            // If we divide both the numerator and denominator by their GCD, we get the fraction in its lowest terms.

            // Keep track of the original numbers for explanation.
            let original_num = num;
            let original_den = den;

            // Loop to find and divide by common factors.
            loop {
                // Calculate the greatest common divisor (GCD) of the current numerator and denominator.
                let common_factor = hcf(num, den);

                // If the common factor is 1, it means there are no more common factors other than 1.
                // So, the fraction is already in its lowest terms.
                if common_factor == 1 {
                    println!("{} and {} have no common factors other than 1.", num, den);
                    println!("The fraction {}/{} in its lowest terms is: {}/{}", original_num, original_den, num, den);
                    break; // Exit the loop
                } else {
                    // If there is a common factor greater than 1, divide both numerator and denominator by it.
                    println!("Divide {} and {} by their common factor {}.", num, den, common_factor);
                    num /= common_factor;
                    den /= common_factor;
                    println!("Result: {}/{}", num, den);
                }
            }
        } else {
            println!("Invalid numbers. Please enter valid integers for numerator and denominator.");
        }
        println!("\nEnter another fraction or type 'exit' to quit.");
    }
}

// Helper function to calculate the Highest Common Factor (HCF) or Greatest Common Divisor (GCD).
// This uses the Euclidean algorithm.
fn hcf(a: u32, b: u32) -> u32 {
    let mut temp_a = a;
    let mut temp_b = b;
    while temp_b != 0 {
        let t = temp_b;
        temp_b = temp_a % temp_b;
        temp_a = t;
    }
    temp_a
}
