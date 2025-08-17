use std::io::{self, Write};

/// Rounds a floating-point number to a specified number of decimal places.
///
/// # Arguments
///
/// * `n` - The number to round.
/// * `decimal_places` - The number of decimal places to round to.
fn round_to_decimal_places(n: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (n * multiplier).round() / multiplier
}

/// Prompts the user for input, reads it, and parses it into the specified type.
///
/// # Arguments
///
/// * `prompt` - The message to display to the user.
fn get_user_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    println!("=== Decimal Places Rounding ===");

    let number: f64 = get_user_input("Enter a number to round: ");
    let decimal_places: u32 = get_user_input("Enter the number of decimal places: ");

    let rounded_number = round_to_decimal_places(number, decimal_places);

    println!("\nThe rounded number is: {}", rounded_number);
}
