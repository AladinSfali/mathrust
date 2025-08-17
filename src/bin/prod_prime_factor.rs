// Import the necessary libraries from the standard library.
use std::io; // Used for handling user input.
use std::collections::BTreeMap; // Used for storing the prime factors and their exponents in a sorted map.

// This function calculates the prime factorization of a given number `n`.
// It returns a BTreeMap where the keys are the prime factors and the values are their exponents.
fn prime_factorization(mut n: u32) -> BTreeMap<u32, u32> {
    // Create a new, empty BTreeMap to store the prime factors.
    let mut factors = BTreeMap::new();
    // Start with the first prime number, 2.
    let mut d = 2;

    // Loop until the number `n` is reduced to 1.
    while n > 1 {
        // Check if the current divisor `d` is a factor of `n`.
        while n % d == 0 {
            // If `d` is a factor, increment its count in the `factors` map.
            // `entry(d)` gets the entry for the key `d`, and `or_insert(0)` inserts 0 if the key is not present.
            // The `*` dereferences the value to increment it.
            *factors.entry(d).or_insert(0) += 1;
            // Divide `n` by `d` to remove the factor.
            n /= d;
        }
        // Move to the next potential divisor.
        d += 1;
        // Optimization: if `d*d` is greater than `n`, then the remaining `n` must be prime.
        if d * d > n {
            if n > 1 {
                // If `n` is still greater than 1, it is a prime factor itself.
                *factors.entry(n).or_insert(0) += 1;
            }
            // Exit the loop since we have found all prime factors.
            break;
        }
    }
    // Return the map of prime factors.
    factors
}

// The main function, where the program execution begins.
fn main() {
    // Prompt the user to enter a number.
    println!("Enter a number to find its prime factorization:");

    // Create a mutable string to store the user's input.
    let mut input = String::new();
    // Read the line of input from the user.
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the user's input into a 32-bit unsigned integer.
    let number: u32 = match input.trim().parse() {
        // If parsing is successful, assign the number to the `number` variable.
        Ok(num) => num,
        // If parsing fails, print an error message and exit the program.
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // The prime factorization is not defined for numbers less than or equal to 1.
    if number <= 1 {
        println!("The number must be greater than 1.");
        return;
    }

    // Call the `prime_factorization` function to get the prime factors of the number.
    let factors = prime_factorization(number);

    // Format the prime factors into a string like "2^2 * 3".
    let result = factors
        .iter()
        .map(|(base, exp)| {
            // If the exponent is 1, just show the base.
            if *exp == 1 {
                format!("{}", base)
            // If the exponent is greater than 1, show "base^exponent".
            } else {
                format!("{}^{}", base, exp)
            }
        })
        // Collect the formatted strings into a vector.
        .collect::<Vec<String>>()
        // Join the elements of the vector with " * ".
        .join(" * ");

    // Print the final result to the console.
    println!("The prime factorization of {} is: {}", number, result);
}