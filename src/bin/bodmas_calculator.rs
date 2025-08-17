// BODMAS Calculator
// This program evaluates mathematical expressions using the `meval` crate,
// which respects the BODMAS/PEMDAS order of operations.

use std::io::{self, Write};

fn main() {
    println!("=== BODMAS Calculator ===");
    println!("Enter mathematical expressions like: 20 - 3 * 4 or 30 / (15 - 12)");
    println!("Supported operations: +, -, *, /, %, ^, ( )");
    println!("Type 'quit' to exit.\n");

    loop {
        print!("Enter expression: ");
        // Ensure the prompt is displayed immediately.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        // Use the `meval` crate to evaluate the expression.
        // It's a powerful and safe way to handle mathematical expressions.
        match meval::eval_str(input) {
            Ok(result) => {
                // Check if the result is an integer.
                if result.fract() == 0.0 {
                    println!("Result: {}\n", result as i64);
                } else {
                    println!("Result: {}\n", result);
                }
            }
            Err(e) => println!("Error: {}\n", e),
        }
    }
}
