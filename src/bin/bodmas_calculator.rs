// Simple BODMAS Calculator for High School Students
// This program evaluates mathematical expressions following BODMAS order
// BODMAS: Brackets, Orders (powers), Division, Multiplication, Addition, Subtraction

use std::io;
use std::io::Write;

// This is our main function - where the program starts
fn main() {
    println!("=== Simple BODMAS Calculator ===");
    println!("Enter mathematical expressions like: 20-3*4 or 30/(15-12)");
    println!("Supported operations: +, -, *, /, ( )");
    println!("Type 'quit' to exit\n");

    // Keep running until user wants to quit
    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap(); // Make sure the prompt appears

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim(); // Remove whitespace

        // Check if user wants to quit
        if input == "quit" {
            println!("Goodbye!");
            break;
        }

        // Try to evaluate the expression
        match evaluate_expression(input) {
            Ok(result) => println!("Result: {}\n", result),
            Err(error) => println!("Error: {}\n", error),
        }
    }
}

// This function takes a string expression and returns the calculated result
// It uses a technique called "recursive descent parsing"
fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // Remove all spaces to make parsing easier
    let expression = expression.replace(" ", "");
    
    // Convert the string into a list of characters we can work with
    let chars: Vec<char> = expression.chars().collect();
    let mut position = 0; // Keep track of where we are in the expression
    
    // Start parsing from the lowest precedence operations (addition/subtraction)
    parse_addition_subtraction(&chars, &mut position)
}

// BODMAS Step 5 & 6: Handle Addition and Subtraction (lowest precedence)
// These operations are performed last, so we parse them first in our recursive approach
fn parse_addition_subtraction(chars: &Vec<char>, position: &mut usize) -> Result<f64, String> {
    // First, get the result of higher precedence operations
    let mut result = parse_multiplication_division(chars, position)?;
    
    // Keep processing + and - operations from left to right
    while *position < chars.len() {
        let operator = chars[*position];
        
        if operator == '+' || operator == '-' {
            *position += 1; // Move past the operator
            let right_operand = parse_multiplication_division(chars, position)?;
            
            // Perform the operation
            if operator == '+' {
                result += right_operand;
                println!("  Adding: {} + {} = {}", result - right_operand, right_operand, result);
            } else {
                result -= right_operand;
                println!("  Subtracting: {} - {} = {}", result + right_operand, right_operand, result);
            }
        } else {
            break; // No more + or - operators
        }
    }
    
    Ok(result)
}

// BODMAS Step 3 & 4: Handle Multiplication and Division
// These have higher precedence than addition/subtraction
fn parse_multiplication_division(chars: &Vec<char>, position: &mut usize) -> Result<f64, String> {
    // First, get the result of even higher precedence operations (parentheses and numbers)
    let mut result = parse_parentheses_and_numbers(chars, position)?;
    
    // Keep processing * and / operations from left to right
    while *position < chars.len() {
        let operator = chars[*position];
        
        if operator == '*' || operator == '/' {
            *position += 1; // Move past the operator
            let right_operand = parse_parentheses_and_numbers(chars, position)?;
            
            // Perform the operation
            if operator == '*' {
                result *= right_operand;
                println!("  Multiplying: {} * {} = {}", result / right_operand, right_operand, result);
            } else {
                if right_operand == 0.0 {
                    return Err("Cannot divide by zero!".to_string());
                }
                result /= right_operand;
                println!("  Dividing: {} / {} = {}", result * right_operand, right_operand, result);
            }
        } else {
            break; // No more * or / operators
        }
    }
    
    Ok(result)
}

// BODMAS Step 1: Handle Brackets (Parentheses) and Numbers
// Brackets have the highest precedence, so they're handled first
fn parse_parentheses_and_numbers(chars: &Vec<char>, position: &mut usize) -> Result<f64, String> {
    // Skip any whitespace (though we removed it earlier)
    while *position < chars.len() && chars[*position] == ' ' {
        *position += 1;
    }
    
    if *position >= chars.len() {
        return Err("Unexpected end of expression".to_string());
    }
    
    let current_char = chars[*position];
    
    // Handle opening parenthesis - highest priority in BODMAS
    if current_char == '(' {
        println!("  Found opening bracket at position {}", *position);
        *position += 1; // Move past the '('
        
        // Recursively evaluate everything inside the parentheses
        let result = parse_addition_subtraction(chars, position)?;
        
        // Make sure we have a closing parenthesis
        if *position >= chars.len() || chars[*position] != ')' {
            return Err("Missing closing parenthesis".to_string());
        }
        
        println!("  Found closing bracket, result inside: {}", result);
        *position += 1; // Move past the ')'
        Ok(result)
    }
    // Handle negative numbers
    else if current_char == '-' {
        *position += 1; // Move past the '-'
        let number = parse_parentheses_and_numbers(chars, position)?;
        Ok(-number)
    }
    // Handle regular numbers
    else if current_char.is_numeric() || current_char == '.' {
        parse_number(chars, position)
    }
    else {
        Err(format!("Unexpected character: {}", current_char))
    }
}

// Helper function to parse numbers (including decimal numbers)
fn parse_number(chars: &Vec<char>, position: &mut usize) -> Result<f64, String> {
    let start_position = *position;
    
    // Keep reading digits and decimal points
    while *position < chars.len() {
        let ch = chars[*position];
        if ch.is_numeric() || ch == '.' {
            *position += 1;
        } else {
            break;
        }
    }
    
    // Convert the characters we found into a number
    let number_str: String = chars[start_position..*position].iter().collect();
    
    // Try to parse the string as a floating-point number
    match number_str.parse::<f64>() {
        Ok(number) => {
            println!("  Parsed number: {}", number);
            Ok(number)
        }
        Err(_) => Err(format!("Invalid number: {}", number_str))
    }
}

