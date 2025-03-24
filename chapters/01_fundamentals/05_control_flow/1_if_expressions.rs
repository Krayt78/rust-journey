// If Expressions in Rust
//
// This program demonstrates how if expressions work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions

fn main() {
    println!("Exploring If Expressions in Rust!");
    
    //------------------------------------------------------
    // IF EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- Basic If Expression ---");
    
    let number = 5;
    
    if number < 0 {
        println!("The number is negative");
    } else if number == 0 {
        println!("The number is zero");
    } else {
        println!("The number is positive");
    }
    
    // If expressions with return values
    println!("\n--- If Expression with Return Values ---");
    
    let condition = true;
    let value = if condition { "condition is true" } else { "condition is false" };
    println!("The value is: {}", value);
    
    // If expressions must return the same type from all branches
    println!("\n--- Types Must Match ---");
    
    let number = if condition { 5 } else { 6 };
    println!("The number is: {}", number);
    
    // The following would not compile because the types don't match:
    // let mismatch = if condition { 5 } else { "six" };
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the if expression
    pub fn challenge_if_expression() -> Result<(), String> {
        let number = 7;
        
        // This if expression should check if number is greater than 5
        // but the condition is wrong. Fix it.
        let message = if number < 5 {
            "number is greater than 5"
        } else {
            "number is 5 or less"
        };
        
        if message != "number is greater than 5" {
            return Err("The message should indicate number is greater than 5".to_string());
        }
        
        Ok(())
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning If Expression challenges...");
        
        // Challenge 1: Fix if expression
        if let Err(e) = challenges::challenge_if_expression() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the if expression!");
        
        println!("All If Expression challenges completed successfully!");
        Ok(())
    }
} 