// This program helps you find a common denominator for two or more fractions.
// Finding a common denominator is the first step to comparing or adding/subtracting fractions.

use std::io;

// Helper function to calculate the greatest common divisor (GCD) of two numbers.
// This is needed by the LCM function.
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

// Helper function to calculate the least common multiple (LCM) of two numbers.
// The LCM of the denominators will be our common denominator.
fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / hcf(a, b)) * b
    }
}

// This is the main function where our program starts.
fn main() {
    println!("Ordering Fractions by Common Denominator");
    println!("Enter fractions separated by commas (e.g., 5/6, 3/8)");

    // This loop allows the user to process multiple sets of fractions.
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let clean_input = input.trim();

        if clean_input == "exit" {
            break;
        }

        // Parse the input string into individual fractions.
        let fractions_str: Vec<&str> = clean_input.split(',').collect();
        let mut fractions: Vec<(u32, u32)> = Vec::new();

        for f_str in fractions_str {
            let parts: Vec<&str> = f_str.trim().split('/').collect();
            if parts.len() == 2 {
                if let (Ok(num), Ok(den)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    if den == 0 {
                        println!("Error: Denominator cannot be zero for fraction {}.", f_str);
                        fractions.clear(); // Clear to avoid processing invalid data
                        break;
                    }
                    fractions.push((num, den));
                } else {
                    println!("Error: Invalid number in fraction {}.", f_str);
                    fractions.clear();
                    break;
                }
            } else {
                println!("Error: Invalid fraction format {}. Expected numerator/denominator.", f_str);
                fractions.clear();
                break;
            }
        }

        if fractions.is_empty() {
            println!("Please enter valid fractions.");
            continue;
        }

        println!("\nFinding a common denominator for: {:?}", fractions);

        // Step 1: Find the Least Common Multiple (LCM) of all denominators.
        // This LCM will be our common denominator.
        let mut common_denominator = fractions[0].1; // Start with the first denominator
        for i in 1..fractions.len() {
            common_denominator = lcm(common_denominator, fractions[i].1);
        }
        println!("1. The Least Common Multiple (LCM) of the denominators is: {}", common_denominator);

        // Step 2: Rewrite each fraction with the common denominator.
        println!("2. Rewriting each fraction with the common denominator:");
        for (num, den) in fractions {
            // How many times do we need to multiply the original denominator to get the common denominator?
            let multiplier = common_denominator / den;
            // Multiply both the numerator and denominator by this multiplier.
            let new_numerator = num * multiplier;
            println!("   {}/{} becomes {}/{}", num, den, new_numerator, common_denominator);
        }

        println!("\nEnter another set of fractions or type 'exit' to quit.");
    }
}
