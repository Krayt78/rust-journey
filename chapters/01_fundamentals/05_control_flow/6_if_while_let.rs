// If Let and While Let Expressions in Rust
//
// This program demonstrates how if let and while let expressions work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch06-03-if-let.html

fn main() {
    println!("Exploring If Let and While Let in Rust!");
    
    //------------------------------------------------------
    // IF LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- If Let Expression ---");
    
    // Using if let to simplify matching on one pattern
    let some_value: Option<i32> = Some(3);
    
    // Regular match expression
    println!("Using match:");
    match some_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value"),
    }
    
    // Same thing with if let
    println!("Using if let:");
    if let Some(value) = some_value {
        println!("The value is: {}", value);
    } else {
        println!("No value");
    }
    
    //------------------------------------------------------
    // IF LET WITH COMPLEX PATTERNS
    //------------------------------------------------------
    println!("\n--- If Let with Complex Patterns ---");
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 10, y: 20 };
    
    if let Point { x, y: 20 } = point {
        println!("Point has y-coordinate 20 and x-coordinate {}", x);
    } else {
        println!("Point doesn't have y-coordinate 20");
    }
    
    //------------------------------------------------------
    // WHILE LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- While Let Expression ---");
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Using while let to process items while a pattern matches
    println!("Popping values from the stack:");
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    //------------------------------------------------------
    // WHILE LET WITH ITERATORS
    //------------------------------------------------------
    println!("\n--- While Let with Iterators ---");
    
    let numbers = vec![10, 20, 30];
    let mut iter = numbers.iter();
    
    while let Some(number) = iter.next() {
        println!("Got: {}", number);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the if let expression to extract the value correctly
    pub fn challenge_if_let() -> Result<(), String> {
        let optional: Option<i32> = Some(42);
        let mut value = 0;
        
        // This if let should extract the value from optional and multiply it by 2
        // Fix it to correctly extract and transform the value
        if let None = optional {
            value = 84;
        }
        
        if value != 84 {
            return Err(format!("Value should be 84, but got {}", value));
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
        println!("\nRunning If Let challenges...");
        
        // Challenge: Fix if let expression
        if let Err(e) = challenges::challenge_if_let() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the if let expression!");
        
        println!("All If Let challenges completed successfully!");
        Ok(())
    }
} 