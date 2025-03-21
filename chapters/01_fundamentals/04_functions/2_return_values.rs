// Return Values in Rust Functions
//
// This program demonstrates how functions can return values.

fn main() {
    println!("Exploring Function Return Values in Rust!");
    
    // Calling a function that returns a value
    let result = add(5, 7);
    println!("5 + 7 = {}", result);
    
    // Using the returned value in an expression
    let doubled = result * 2;
    println!("Doubled: {}", doubled);
    
    // Calling a function that doesn't explicitly return a value
    // Such functions implicitly return the unit type ()
    let nothing = say_hello();
    println!("Value of nothing: {:?}", nothing);
    
    // Using returned values directly in expressions
    println!("15 + 27 = {}", add(15, 27));
    
    // Multiple return values using tuples
    let (product, difference) = multiply_and_subtract(10, 4);
    println!("10 * 4 = {}, 10 - 4 = {}", product, difference);
    
    // Early returns with conditionals
    let num = 42;
    let result = is_positive_or_zero(num);
    println!("{} is positive or zero: {}", num, result);
    
    let negative = -10;
    let result = is_positive_or_zero(negative);
    println!("{} is positive or zero: {}", negative, result);
}

// Function that returns a value (i32)
fn add(a: i32, b: i32) -> i32 {
    // In Rust, the last expression in a function is returned
    // Note: no semicolon at the end, making it an expression, not a statement
    a + b
}

// Function that doesn't explicitly return a value
fn say_hello() {
    // This function returns the unit type () implicitly
    println!("Hello!");
}

// Function that returns multiple values using a tuple
fn multiply_and_subtract(a: i32, b: i32) -> (i32, i32) {
    let product = a * b;
    let difference = a - b;
    // Return a tuple containing both values
    (product, difference)
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: Fix this function to return the square of the input number
    fn square(num: i32) {
        num * num;
    }
    
    // TODO: Fix this function to return a boolean indicating if a number is even
    fn is_even(num: i32) {
        if num % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }
    
    // TODO: Fix this function to return both the sum and product of two numbers
    fn calculate(a: i32, b: i32) -> (i32, i32) {
        let sum = a + b;
        let product = a * b;
        // Something is wrong with this return statement
        sum, product
    }
    
    // TODO: Fix this function to find the maximum of three numbers
    fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
        // This implementation is incorrect
        if a > b {
            a
        } else {
            b
        }
        // What about c?
    }
    
    // TODO: Fix and call the functions above
    pub fn run_functions() -> Result<(), String> {
        // Call and test square function
        let squared = square(5);
        if squared != 25 {
            return Err(format!("square(5) returned {}, expected 25", squared));
        }
        
        // Call and test is_even function
        let even = is_even(4);
        let odd = is_even(7);
        if !even || odd {
            return Err("is_even function returning incorrect results".to_string());
        }
        
        // Call and test calculate function
        let (sum, product) = calculate(3, 4);
        if sum != 7 || product != 12 {
            return Err(format!("calculate(3, 4) returned ({}, {}), expected (7, 12)", sum, product));
        }
        
        // Call and test max_of_three function
        let tests = [
            (5, 9, 3, 9),  // b is max
            (10, 4, 7, 10), // a is max
            (2, 5, 8, 8),   // c is max
            (5, 5, 5, 5),   // all equal
        ];
        
        for (a, b, c, expected) in tests {
            let max = max_of_three(a, b, c);
            if max != expected {
                return Err(format!("max_of_three({}, {}, {}) returned {}, expected {}", 
                                   a, b, c, max, expected));
            }
        }
        
        println!("All functions with return values work correctly!");
        Ok(())
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Function Return Values challenges...");
        
        challenges::run_functions()
    }
}