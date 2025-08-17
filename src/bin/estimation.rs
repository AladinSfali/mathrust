// Import the necessary libraries from the standard library and the meval crate.
use std::io; // Used for handling user input.
extern crate meval;

// This function rounds a floating-point number `n` to one significant figure.
fn round_to_one_sf(n: f64) -> f64 {
    // This is the same logic as in the significant_figures.rs program, but hardcoded to 1 sf.
    if n == 0.0 {
        return 0.0;
    }
    let position = n.abs().log10().floor();
    let multiplier = 10.0_f64.powi(1 - position as i32 - 1);
    (n * multiplier).round() / multiplier
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter a mathematical expression.
    println!("Enter a mathematical expression to estimate (e.g., (9.7*326)/(1.823*5.325)):");

    // Create a mutable string to store the user's input.
    let mut expression = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut expression).expect("Failed to read line");

    // Manually identify numbers in the expression and round them to one significant figure.
    // This is a simplified approach and might not handle all edge cases.
    let mut rounded_expression = expression.clone();
    let mut estimated_parts = Vec::new();

    // This regex is a simplified way to find numbers in the expression.
    let re = regex::Regex::new(r"[0-9]+(\.[0-9]+)?").unwrap();
    for mat in re.find_iter(&expression) {
        let number_str = mat.as_str();
        if let Ok(number) = number_str.parse::<f64>() {
            let rounded_number = round_to_one_sf(number);
            rounded_expression = rounded_expression.replace(number_str, &rounded_number.to_string());
            estimated_parts.push(format!("{} ≈ {}", number_str, rounded_number));
        }
    }

    // Print the rounding steps.
    println!("\nRounding each number to one significant figure:");
    for part in estimated_parts {
        println!("{}", part);
    }

    // Show the estimated expression.
    println!("\nEstimated expression: {}", rounded_expression.trim());

    // Evaluate the estimated expression using the `meval` crate.
    match meval::eval_str(rounded_expression.trim()) {
        Ok(result) => {
            // To show the intermediate steps, we can do a simplified evaluation.
            // This is a very basic example and will not handle operator precedence correctly.
            let parts: Vec<&str> = rounded_expression.trim().split(|c| c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')').filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                let first_op_index = rounded_expression.find(|c: char| c == '+' || c == '-' || c == '*' || c == '/').unwrap_or(0);
                let operator = rounded_expression.chars().nth(first_op_index).unwrap_or(' ');

                let num1_str = parts[0].trim();
                let num2_str = parts[1].trim();

                if let (Ok(num1), Ok(num2)) = (num1_str.parse::<f64>(), num2_str.parse::<f64>()) {
                    let intermediate_result = match operator {
                        '*' => num1 * num2,
                        '/' => num1 / num2,
                        '+' => num1 + num2,
                        '-' => num1 - num2,
                        _ => 0.0,
                    };
                    println!("\nStep 1: {} {} {} = {}", num1, operator, num2, intermediate_result);
                    println!("\nFinal Result ≈ {}", result);
                }
            } else {
                 println!("\nFinal Result ≈ {}", result);
            }
        },
        Err(e) => println!("\nCould not evaluate the expression: {}", e),
    };
}
