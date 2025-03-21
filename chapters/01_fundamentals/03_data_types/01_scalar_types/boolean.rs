// Boolean Type in Rust
//
// The boolean type in Rust has two possible values: true and false.
// Booleans are one byte in size.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type

fn main() {
    println!("Exploring Boolean Types in Rust!");
    
    //------------------------------------------------------
    // BOOLEAN DECLARATIONS
    //------------------------------------------------------
    // Boolean declaration
    let is_active = true;
    let is_greater = 10 > 5; // Expression that evaluates to a boolean
    
    println!("is_active: {}", is_active);
    println!("is_greater: {}", is_greater);
    
    // Explicit type annotation (usually not needed due to type inference)
    let explicit_bool: bool = false;
    println!("explicit_bool: {}", explicit_bool);
    
    //------------------------------------------------------
    // BOOLEAN OPERATORS
    //------------------------------------------------------
    // Boolean operators
    
    // Logical AND (&&)
    let both_true = true && true;  // true
    let one_false = true && false; // false
    
    println!("\nLogical AND (&&):");
    println!("true && true = {}", both_true);
    println!("true && false = {}", one_false);
    
    // Logical OR (||)
    let either_true = true || false;  // true
    let both_false = false || false;  // false
    
    println!("\nLogical OR (||):");
    println!("true || false = {}", either_true);
    println!("false || false = {}", both_false);
    
    // Logical NOT (!)
    let not_true = !true;   // false
    let not_false = !false; // true
    
    println!("\nLogical NOT (!):");
    println!("!true = {}", not_true);
    println!("!false = {}", not_false);
    
    //------------------------------------------------------
    // COMBINING OPERATORS
    //------------------------------------------------------
    // Complex boolean expressions
    let complex_expression = (true && false) || (true && !false);
    println!("\nComplex expression: (true && false) || (true && !false) = {}", complex_expression);
    
    //------------------------------------------------------
    // CONTROL FLOW WITH BOOLEANS
    //------------------------------------------------------
    // Using booleans in control flow
    let number = 7;
    
    if number % 2 == 0 {
        println!("\n{} is even", number);
    } else {
        println!("\n{} is odd", number);
    }
    
    // Boolean as a condition
    let is_evening = true;
    
    if is_evening {
        println!("Good evening!");
    } else {
        println!("Good day!");
    }
    
    //------------------------------------------------------
    // SHORT-CIRCUIT EVALUATION
    //------------------------------------------------------
    // Short-circuit evaluation
    // In a chain of && operations, if one operand is false, the rest aren't evaluated
    // In a chain of || operations, if one operand is true, the rest aren't evaluated
    
    println!("\nShort-circuit evaluation:");
    
    let x = 5;
    let y = 10;
    
    // The second condition is only evaluated if the first one is true
    if x > 0 && y / x > 1 {
        println!("y/x is greater than 1");
    }
    
    // The second condition is only evaluated if the first one is false
    if x > 10 || y > 5 {
        println!("At least one condition is true");
    }
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These boolean expressions have errors
    
    // Error 1: Using wrong operator
    pub fn challenge_boolean_expressions() -> Result<(), String> {
        let a = true;
        let b = false;
        
        // This should be true, but the expression is wrong
        let expression1 = a & b; // Using bitwise AND instead of logical AND
        
        // This should be false, but the expression is wrong
        let expression2 = a or b; // Using invalid "or" keyword instead of ||
        
        // This should be true, but the expression is wrong
        let expression3 = not a; // Using invalid "not" keyword instead of !
        
        if !expression1 {
            // This check will pass if expression1 is fixed to be false
        } else {
            return Err("expression1 should be false".to_string());
        }
        
        if expression2 {
            // This check will pass if expression2 is fixed to be true
        } else {
            return Err("expression2 should be true".to_string());
        }
        
        if expression3 {
            // This check will pass if expression3 is fixed to be false
        } else {
            return Err("expression3 should be false".to_string());
        }
        
        Ok(())
    }
    
    // TODO: This function should return true if n is both even and positive
    pub fn is_even_and_positive(n: i32) -> bool {
        // There's an error in this expression
        n > 0 || n % 2 == 0 // Should use AND, not OR
    }
    
    // TODO: This function should determine if a year is a leap year using the rules:
    // 1. Years divisible by 4 are leap years
    // 2. However, years divisible by 100 are NOT leap years
    // 3. Unless they are also divisible by 400, then they ARE leap years
    pub fn is_leap_year(year: u32) -> bool {
        // There's an error in this expression
        year % 4 == 0 && year % 100 == 0 || year % 400 == 0
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning boolean challenges...");
        
        // Challenge 1: Fix boolean expressions
        if let Err(e) = challenges::challenge_boolean_expressions() {
            return Err(format!("Boolean expressions challenge failed: {}", e));
        }
        println!("Successfully fixed the boolean expressions!");
        
        // Challenge 2: Fix is_even_and_positive function
        if challenges::is_even_and_positive(-2) {
            return Err("is_even_and_positive(-2) should be false (negative number)".to_string());
        }
        if challenges::is_even_and_positive(3) {
            return Err("is_even_and_positive(3) should be false (odd number)".to_string());
        }
        if !challenges::is_even_and_positive(4) {
            return Err("is_even_and_positive(4) should be true (even and positive)".to_string());
        }
        println!("Successfully fixed the is_even_and_positive function!");
        
        // Challenge 3: Fix is_leap_year function
        if !challenges::is_leap_year(2020) {
            return Err("2020 should be a leap year (divisible by 4)".to_string());
        }
        if challenges::is_leap_year(2100) {
            return Err("2100 should not be a leap year (divisible by 100 but not 400)".to_string());
        }
        if !challenges::is_leap_year(2000) {
            return Err("2000 should be a leap year (divisible by 400)".to_string());
        }
        println!("Successfully fixed the is_leap_year function!");
        
        println!("All boolean challenges completed successfully!");
        Ok(())
    }
}