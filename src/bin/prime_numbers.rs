use std::io;

// This function checks if a given number is prime.
fn is_prime(number: u32) -> bool { // Defines a function named 'is_prime' that takes a 'u32' and returns a 'bool' [1-4]
    // Numbers less than or equal to 1 are not prime.
    if number <= 1 { // Uses an 'if' expression for conditional branching [5, 6]
        return false; // Returns 'false' if the condition is met [4, 7]
    }

    // Check for divisibility from 2 up to half of the number.
    // The 'for' loop iterates over a range of numbers [8-10]
    for i in 2..(number / 2 + 1) { // Uses integer division and range syntax [11-13]
        if number % i == 0 { // Uses the remainder operator '%' to check for divisibility [11, 13]
            return false; // If divisible, it's not prime, so return 'false' [4, 7]
        }
    }

    // If no divisors were found, the number is prime.
    true // This is an expression that evaluates to 'true' and is implicitly returned [4, 7, 14, 15]
}

fn main() {
    println!("Enter a set of numbers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<u32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("No valid numbers were entered.");
        return;
    }

    println!("\nChecking for prime numbers in the list:");

    for num in numbers {
        if is_prime(num) {
            println!("{} is a prime number.", num);
        } else {
            println!("{} is not a prime number.", num);
        }
    }
}