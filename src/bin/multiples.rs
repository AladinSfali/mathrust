// GCSE Math - Section 1.2: Multiples Calculator
// This program demonstrates three different ways to calculate and display multiples
// 
// What are multiples?
// Multiples of a number are the results you get when you multiply that number
// by positive integers (1, 2, 3, 4, 5, etc.)
// For example: Multiples of 3 are 3, 6, 9, 12, 15, 18, 21, 24, 27, 30...

use std::io::{self, Write};

fn main() {
    println!("=== GCSE Mathematics: Multiples Calculator ===");
    println!("Section 1.2 - Multiples and Factors\n");
    
    // Main program loop - keeps running until user chooses to exit
    loop {
        display_menu();
        
        // Get user's choice
        let choice = get_user_input("Enter your choice (1-4): ");
        
        match choice.trim() {
            "1" => option_one_individual_number(),
            "2" => option_two_list_of_numbers(),
            "3" => option_three_range_of_numbers(),
            "4" => {
                println!("Thank you for using the Multiples Calculator!");
                break;
            }
            _ => println!("Invalid choice! Please enter 1, 2, 3, or 4.\n"),
        }
    }
}

// Function to display the main menu options
fn display_menu() {
    println!("Choose an option:");
    println!("1. Find multiples of a single number");
    println!("2. Find multiples of a list of numbers");
    println!("3. Find multiples of numbers in a range");
    println!("4. Exit");
    println!();
}

// Helper function to get user input with a prompt
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt appears immediately
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

// OPTION 1: Calculate multiples of a single number
fn option_one_individual_number() {
    println!("\n=== Option 1: Multiples of a Single Number ===");
    println!("Enter a positive integer to find its first 20 multiples:");
    
    // Get the number from user
    let input = get_user_input("Number: ");
    
    // Try to parse the input as a positive integer
    match input.trim().parse::<u32>() {
        Ok(number) => {
            if number == 0 {
                println!("Error: Please enter a positive number (greater than 0).\n");
                return;
            }
            
            println!("\nThe first 20 multiples of {} are:", number);
            calculate_and_display_multiples(number, 20);
        }
        Err(_) => {
            println!("Error: Please enter a valid positive integer.\n");
        }
    }
    
    println!(); // Add spacing before returning to menu
}

// OPTION 2: Calculate multiples of a list of numbers
fn option_two_list_of_numbers() {
    println!("\n=== Option 2: Multiples of a List of Numbers ===");
    println!("Enter numbers separated by commas (e.g., 3,5,7):");
    
    let input = get_user_input("Numbers: ");
    
    // Split the input by commas and try to parse each number
    let numbers: Result<Vec<u32>, _> = input
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<u32>())
        .collect();
    
    match numbers {
        Ok(number_list) => {
            if number_list.is_empty() {
                println!("Error: Please enter at least one number.\n");
                return;
            }
            
            // Check if any number is zero
            if number_list.contains(&0) {
                println!("Error: All numbers must be positive (greater than 0).\n");
                return;
            }
            
            println!("\nCalculating multiples for each number:");
            for number in number_list {
                println!("\nFirst 20 multiples of {}:", number);
                calculate_and_display_multiples(number, 20);
            }
        }
        Err(_) => {
            println!("Error: Please enter valid positive integers separated by commas.\n");
        }
    }
    
    println!(); // Add spacing before returning to menu
}

// OPTION 3: Calculate multiples of numbers in a range
fn option_three_range_of_numbers() {
    println!("\n=== Option 3: Multiples of Numbers in a Range ===");
    println!("Enter the start and end of the range:");
    
    // Get start number
    let start_input = get_user_input("Start number: ");
    let start = match start_input.trim().parse::<u32>() {
        Ok(num) => {
            if num == 0 {
                println!("Error: Start number must be positive (greater than 0).\n");
                return;
            }
            num
        }
        Err(_) => {
            println!("Error: Please enter a valid positive integer for start.\n");
            return;
        }
    };
    
    // Get end number
    let end_input = get_user_input("End number: ");
    let end = match end_input.trim().parse::<u32>() {
        Ok(num) => {
            if num == 0 {
                println!("Error: End number must be positive (greater than 0).\n");
                return;
            }
            num
        }
        Err(_) => {
            println!("Error: Please enter a valid positive integer for end.\n");
            return;
        }
    };
    
    // Validate range
    if start > end {
        println!("Error: Start number ({}) cannot be greater than end number ({}).\n", start, end);
        return;
    }
    
    // Check if range is too large (to prevent memory issues)
    if end - start > 50 {
        println!("Warning: Range is quite large ({}). This might take a while.", end - start + 1);
        let confirm = get_user_input("Continue? (y/n): ");
        if confirm.trim().to_lowercase() != "y" {
            println!("Operation cancelled.\n");
            return;
        }
    }
    
    println!("\nCalculating multiples for numbers from {} to {}:", start, end);
    
    // Calculate multiples for each number in the range
    for number in start..=end {
        println!("\nFirst 20 multiples of {}:", number);
        calculate_and_display_multiples(number, 20);
        
        // Add a small pause for very large ranges to make output readable
        if end - start > 10 {
            println!("Press Enter to continue to next number...");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
    
    println!(); // Add spacing before returning to menu
}

// Helper function to calculate and display multiples of a number
// This function takes a number and how many multiples to show
fn calculate_and_display_multiples(number: u32, count: usize) {
    println!("  Calculation method: {} × 1, {} × 2, {} × 3, ...", number, number, number);
    
    let mut multiples = Vec::new();
    
    // Calculate the multiples
    for i in 1..=count {
        let multiple = number * (i as u32);
        multiples.push(multiple);
        
        // Show the calculation for the first few multiples to be educational
        if i <= 3 {
            println!("  {} × {} = {}", number, i, multiple);
        }
    }
    
    // Display all multiples in a formatted way
    println!("  All {} multiples:", count);
    println!("  [{}]", 
        multiples.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    
    // Educational note about patterns
    if number <= 10 {
        print_educational_notes(number, &multiples);
    }
}

// Function to provide educational insights about multiplication patterns
fn print_educational_notes(number: u32, multiples: &[u32]) {
    println!("  📚 Educational Notes:");
    
    match number {
        2 => println!("     • All multiples of 2 are even numbers!"),
        5 => println!("     • All multiples of 5 end in 0 or 5!"),
        10 => println!("     • All multiples of 10 end in 0!"),
        3 => {
            println!("     • The digits of multiples of 3 add up to a multiple of 3!");
            println!("     • Example: 12 → 1+2=3, 15 → 1+5=6, 18 → 1+8=9");
        }
        4 => println!("     • All multiples of 4 are divisible by 2 twice!"),
        6 => println!("     • Multiples of 6 are divisible by both 2 and 3!"),
        8 => println!("     • All multiples of 8 are even and divisible by 4!"),
        9 => {
            println!("     • The digits of multiples of 9 add up to a multiple of 9!");
            println!("     • Example: 18 → 1+8=9, 27 → 2+7=9, 36 → 3+6=9");
        }
        _ => {
            // Check if it's a perfect square
            let sqrt = (number as f64).sqrt();
            if sqrt == sqrt.floor() {
                println!("     • {} is a perfect square ({} × {})!", number, sqrt as u32, sqrt as u32);
            }
        }
    }
    
    // Show the pattern in differences
    if multiples.len() >= 3 {
        let difference = multiples[1] - multiples[0];
        println!("     • Each multiple increases by {} (the original number)!", difference);
    }
}

// Additional helper function to demonstrate mathematical concepts
#[allow(dead_code)] // This function is for educational purposes
fn demonstrate_multiple_properties() {
    println!("=== Mathematical Properties of Multiples ===");
    println!("1. Every positive integer has infinitely many multiples");
    println!("2. The smallest multiple of any positive integer is the number itself");
    println!("3. Zero is a multiple of every integer");
    println!("4. If 'a' is a multiple of 'b', then 'a' is divisible by 'b'");
    println!("5. The set of multiples of a number forms an arithmetic sequence");
    println!();
}
