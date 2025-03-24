// Match Expressions in Rust
//
// This program demonstrates how match expressions work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch06-02-match.html

fn main() {
    println!("Exploring Match Expressions in Rust!");
    
    //------------------------------------------------------
    // BASIC MATCH EXPRESSION
    //------------------------------------------------------
    println!("\n--- Basic Match Expression ---");
    
    let dice_roll = 4;
    
    match dice_roll {
        1 => println!("You rolled a one!"),
        2 => println!("You rolled a two!"),
        3 => println!("You rolled a three!"),
        4..=6 => println!("You rolled between four and six: {}", dice_roll),
        _ => println!("Invalid dice roll"),
    }
    
    //------------------------------------------------------
    // MATCH WITH BINDING
    //------------------------------------------------------
    println!("\n--- Match with Binding ---");
    
    let maybe_value: Option<i32> = Some(42);
    
    match maybe_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no value"),
    }
    
    //------------------------------------------------------
    // MATCH WITH MULTIPLE PATTERNS
    //------------------------------------------------------
    println!("\n--- Match with Multiple Patterns ---");
    
    let grade = 'B';
    
    match grade {
        'A' => println!("Excellent!"),
        'B' | 'C' => println!("Good job!"),  // Multiple patterns with |
        'D' => println!("You passed"),
        'F' => println!("Sorry, you failed"),
        _ => println!("Invalid grade"),
    }
    
    //------------------------------------------------------
    // MATCH WITH GUARDS
    //------------------------------------------------------
    println!("\n--- Match with Guards ---");
    
    let score = 85;
    
    match score {
        90..=100 => println!("A"),
        s if s >= 80 => println!("B"),
        s if s >= 70 => println!("C"),
        s if s >= 60 => println!("D"),
        _ => println!("F"),
    }
    
    //------------------------------------------------------
    // MATCH WITH STRUCTS AND DESTRUCTURING
    //------------------------------------------------------
    println!("\n--- Match with Destructuring ---");
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 1, y: 2 };
    
    match point {
        Point { x: 0, y: 0 } => println!("At the origin"),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x, y } => println!("At position ({}, {})", x, y),
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the match expression to handle all cases correctly
    pub fn challenge_match() -> Result<(), String> {
        let value = 42;
        let description;
        
        // This match expression should categorize the number
        // but it's missing cases and has a logic error
        match value {
            1 => description = "one",
            2 => description = "two",
            // Fix the match expression to correctly categorize 42
            _ => description = "small number",
        }
        
        if description != "large number" {
            return Err(format!("42 should be described as 'large number', but got '{}'", description));
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
        println!("\nRunning Match Expression challenges...");
        
        // Challenge: Fix match expression
        if let Err(e) = challenges::challenge_match() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the match expression!");
        
        println!("All Match Expression challenges completed successfully!");
        Ok(())
    }
} 