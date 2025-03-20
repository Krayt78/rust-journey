// Boolean Type in Rust
//
// The boolean type in Rust has two possible values: true and false.
// Booleans are one byte in size.

fn main() {
    println!("Exploring Boolean Types in Rust!");
    
    // Boolean declaration
    let is_active = true;
    let is_greater = 10 > 5; // Expression that evaluates to a boolean
    
    println!("is_active: {}", is_active);
    println!("is_greater: {}", is_greater);
    
    // Explicit type annotation (usually not needed due to type inference)
    let explicit_bool: bool = false;
    println!("explicit_bool: {}", explicit_bool);
    
    // Boolean operators
    
    // Logical AND (&&)
    let both_true = true && true;  // true
    let one_false = true && false; // false
    
    println!("\nLogical AND (&&):");
    println!("true && true = {}", both_true);
    println!("true && false = {}", one_false);
    
    // Logical OR (||)
    let either_true = true || false;  // true
    let both_false = false || false; // false
    
    println!("\nLogical OR (||):");
    println!("true || false = {}", either_true);
    println!("false || false = {}", both_false);
    
    // Logical NOT (!)
    let not_true = !true;  // false
    let not_false = !false; // true
    
    println!("\nLogical NOT (!):");
    println!("!true = {}", not_true);
    println!("!false = {}", not_false);
    
    // Short-circuit evaluation
    // && and || will only evaluate the right side if necessary
    
    println!("\nShort-circuit evaluation:");
    
    let x = false;
    let y = true;
    
    // The right side won't be evaluated because x is false
    let short_circuit_and = x && print_and_return_true();
    println!("x && print_and_return_true() = {}", short_circuit_and);
    
    // The right side will be evaluated because x is false and we need to check y
    let short_circuit_or = x || print_and_return_true();
    println!("x || print_and_return_true() = {}", short_circuit_or);
    
    // Booleans in control flow
    println!("\nBooleans in control flow:");
    if is_active {
        println!("The system is active!");
    } else {
        println!("The system is inactive.");
    }
    
    // Converting other types to booleans
    // Note: Rust does not automatically convert non-boolean types to boolean
    
    let zero = 0;
    let empty_string = "";
    
    // This would error in Rust:
    // if zero { println!("Zero is truthy"); } // Error!
    
    // Instead, you must explicitly compare:
    if zero == 0 {
        println!("Zero is zero!");
    }
    
    if empty_string.is_empty() {
        println!("The string is empty!");
    }
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Helper function for demonstrating short-circuit evaluation
fn print_and_return_true() -> bool {
    println!("  This function was called!");
    true
}

// CHALLENGES: TODO: Fix the broken code in this module
mod challenges {
    // TODO: These boolean expressions have errors
    
    // Error 1: Incorrect boolean operator syntax
    let a = true;
    let b = false;
    let result1 = a and b; // Wrong operator syntax
    
    // Error 2: Missing comparison operator
    let x = 5;
    let y = 10;
    let result2 = x > y || x y; // Missing operator
    
    // Error 3: Trying to use a numerical value as a boolean
    let num = 1;
    let result3 = num || false; // Can't use numbers as booleans
    
    // Error 4: Incorrect precedence understanding
    let result4 = true || false && true; // Misunderstanding of operation precedence
    
    // TODO: This function should return true if n is both even and positive
    pub fn is_even_and_positive(n: i32) -> bool {
        // This implementation has logical errors
        n % 2 == 0 || n > 0
    }
    
    // TODO: This function should determine if a year is a leap year using the rules:
    // 1. Years divisible by 4 are leap years
    // 2. Years divisible by 100 are NOT leap years
    // 3. Years divisible by 400 ARE leap years
    pub fn is_leap_year(year: u32) -> bool {
        // This implementation has logical errors
        if year % 4 == 0 {
            true
        } else if year % 100 == 0 {
            true
        } else if year % 400 == 0 {
            true
        } else {
            false
        }
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning boolean challenges...");
        
        // Challenge 1: Fix boolean expressions
        challenge_boolean_expressions()?;
        
        // Challenge 2: Fix the is_even_and_positive function
        if !challenges::is_even_and_positive(10) {
            return Err("is_even_and_positive(10) should return true".to_string());
        }
        if challenges::is_even_and_positive(-10) {
            return Err("is_even_and_positive(-10) should return false".to_string());
        }
        println!("is_even_and_positive function works correctly!");
        
        // Challenge 3: Fix the leap year function
        if !challenges::is_leap_year(2020) {
            return Err("is_leap_year(2020) should return true".to_string());
        }
        if challenges::is_leap_year(2100) {
            return Err("is_leap_year(2100) should return false".to_string());
        }
        if !challenges::is_leap_year(2000) {
            return Err("is_leap_year(2000) should return true".to_string());
        }
        println!("is_leap_year function works correctly!");
        
        println!("All boolean challenges completed successfully!");
        Ok(())
    }
    
    #[test]
    fn challenge_boolean_expressions() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Boolean expressions challenge completed!");
        Ok(())
    }
}