// This program helps you find the missing number in a pair of equivalent fractions.
// Equivalent fractions are fractions that have the same value, even though they may look different.
// For example, 1/2 is equivalent to 2/4, and 4/8.

use std::io;

// This is the main function where our program starts.
fn main() {
    println!("Equivalent Fraction Calculator");
    println!("Enter the fractions in the format a/b = c/d");
    println!("Use an 'x' for the value you want to find.");
    println!("For example: 12/30 = 4/x");

    // This loop allows the user to solve multiple problems without restarting the program.
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

        // Split the input string into two parts at the "=" sign.
        let parts: Vec<&str> = clean_input.split('=').collect();
        if parts.len() != 2 {
            println!("Invalid format. Please use the format a/b = c/d");
            continue;
        }

        // Split the first fraction into numerator and denominator.
        let first_fraction: Vec<&str> = parts[0].trim().split('/').collect();
        // Split the second fraction into numerator and denominator.
        let second_fraction: Vec<&str> = parts[1].trim().split('/').collect();

        if first_fraction.len() != 2 || second_fraction.len() != 2 {
            println!("Invalid fraction format. Please use a/b = c/d");
            continue;
        }

        // Now we parse the strings into numbers. 'x' will be parsed as an error, which we catch.
        let a = first_fraction[0].parse::<f64>();
        let b = first_fraction[1].parse::<f64>();
        let c = second_fraction[0].parse::<f64>();
        let d = second_fraction[1].parse::<f64>();

        // We check which of the values is the unknown 'x' and solve for it.
        // The logic is based on the cross-multiplication rule: if a/b = c/d, then a*d = b*c.
        if a.is_err() {
            // If 'a' is the unknown value.
            // a = (b * c) / d
            let b_val = b.unwrap();
            let c_val = c.unwrap();
            let d_val = d.unwrap();
            let result = (b_val * c_val) / d_val;
            println!("The equation is x/{} = {}/{}", b_val, c_val, d_val);
            println!("To find x, we calculate ({} * {}) / {}", b_val, c_val, d_val);
            println!("The value of x is: {}", result);
        } else if b.is_err() {
            // If 'b' is the unknown value.
            // b = (a * d) / c
            let a_val = a.unwrap();
            let c_val = c.unwrap();
            let d_val = d.unwrap();
            let result = (a_val * d_val) / c_val;
            println!("The equation is {}/x = {}/{}", a_val, c_val, d_val);
            println!("To find x, we calculate ({} * {}) / {}", a_val, d_val, c_val);
            println!("The value of x is: {}", result);
        } else if c.is_err() {
            // If 'c' is the unknown value.
            // c = (a * d) / b
            let a_val = a.unwrap();
            let b_val = b.unwrap();
            let d_val = d.unwrap();
            let result = (a_val * d_val) / b_val;
            println!("The equation is {}/{} = x/{}", a_val, b_val, d_val);
            println!("To find x, we calculate ({} * {}) / {}", a_val, d_val, b_val);
            println!("The value of x is: {}", result);
        } else if d.is_err() {
            // If 'd' is the unknown value, like in the example 12/30 = 4/x
            // d = (b * c) / a
            let a_val = a.unwrap();
            let b_val = b.unwrap();
            let c_val = c.unwrap();

            // Here we follow the logic from your example.
            // 1. Find what you need to divide by to get from one numerator to the other.
            let divisor = a_val / c_val;
            println!("To get from the first numerator ({}) to the second ({}), you divide by {}", a_val, c_val, divisor);

            // 2. Divide the denominator by the same number.
            let result = b_val / divisor;
            println!("So, we divide the first denominator ({}) by the same number ({})", b_val, divisor);
            println!("The value of x is: {}", result);
        } else {
            println!("No unknown value 'x' found in the equation.");
        }
        println!("\nEnter another problem or type 'exit' to quit.");
    }
}
