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
    
    // Challenge: Write a function that determines if a number is both greater
    // than 10 and even. Test it with a few different values.
    // Your code here:
    
    // Challenge: Use boolean logic to determine if a year is a leap year.
    // Your code here:
}

// Helper function for demonstrating short-circuit evaluation
fn print_and_return_true() -> bool {
    println!("  This function was called!");
    true
}