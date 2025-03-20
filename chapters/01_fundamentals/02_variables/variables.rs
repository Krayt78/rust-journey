// Variables in Rust
//
// This program demonstrates how variables work in Rust, including:
// - Immutable and mutable variables
// - Shadowing
// - Constants

fn main() {
    println!("Exploring Variables in Rust!");
    
    // IMMUTABLE VARIABLES
    // By default, variables in Rust are immutable
    let language = "Rust";
    println!("I'm learning {}", language);
    
    // This would cause a compile error:
    // language = "Rust Programming";
    
    // MUTABLE VARIABLES
    // Use the 'mut' keyword to make a variable mutable
    let mut version = "1.0";
    println!("Starting with version: {}", version);
    
    // Now we can change it
    version = "1.72";
    println!("Updated to version: {}", version);
    
    // SHADOWING
    // We can shadow a variable by using the same name
    let count = 5;
    println!("Original count: {}", count);
    
    // This creates a new variable that shadows the previous one
    let count = count + 5;
    println!("After shadowing: {}", count);
    
    // Shadowing allows changing the type
    let value = "42";
    println!("Value as string: {}", value);
    
    let value = value.parse::<i32>().unwrap();
    println!("Value as integer: {}", value);
    
    // CONSTANTS
    // Constants are always immutable and must have type annotations
    const MAX_USERS: u32 = 100_000;
    println!("Maximum users allowed: {}", MAX_USERS);
    
    // Notice how we use underscores for readability in numbers
    const PI: f64 = 3.141_592_653_589_793;
    println!("Pi: {}", PI);
    
    // VARIABLE SCOPE
    // Variables have a scope, which is the block they're declared in
    {
        // This variable only exists in this block
        let scoped_variable = "I'm only visible inside this block";
        println!("Inside block: {}", scoped_variable);
    }
    // This would cause a compile error:
    // println!("Outside block: {}", scoped_variable);
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// CHALLENGES: TODO: Fix the broken code in this module
mod challenges {
    // TODO: Fix the mutable variable
    pub fn challenge_mutability() -> Result<(), String> {
        // This variable needs to be mutable
        let x = 5;
        x = 10;
        
        if x != 10 {
            return Err("x should be 10".to_string());
        }
        
        Ok(())
    }
    
    // TODO: Fix this shadowing example
    pub fn challenge_shadowing() -> Result<(), String> {
        // We start with a string
        let y = "hello";
        
        // We want to get the length using shadowing
        // Uncomment and fix this line
        // let y = ???
        
        if y != 5 {
            return Err("y should be 5 (the length of 'hello')".to_string());
        }
        
        Ok(())
    }
    
    // TODO: Fix the constant declaration
    pub fn challenge_constants() -> Result<(), String> {
        // Constants should use SCREAMING_SNAKE_CASE and have a type annotation
        const max_score = 100;
        
        if max_score != 100 {
            return Err("MAX_SCORE should be 100".to_string());
        }
        
        Ok(())
    }
    
    // TODO: Fix this complex shadowing example
    pub fn challenge_complex_shadowing() -> Result<(), String> {
        // We start with a number
        let mut value = 10;
        
        // We calculate its square
        let value_squared = value * value;
        
        // We want to shadow value with value_squared
        // But this line has an issue
        value = value_squared;
        
        if value != 100 {
            return Err("value should be 100 (10 squared)".to_string());
        }
        
        Ok(())
    }
    
    // TODO: Fix this boolean shadowing example
    pub fn challenge_boolean_shadowing() -> Result<(), String> {
        // We have a string
        let special = "Rust";
        
        // We want to check if it contains "us"
        // This line has a problem
        let special = special.contains("us");
        
        if !special {
            return Err("special should be true (Rust contains 'us')".to_string());
        }
        
        Ok(())
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Variables challenges...");
        
        // Challenge 1: Fix variable mutability
        if let Err(e) = challenges::challenge_mutability() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the mutable variable!");
        
        // Challenge 2: Fix shadowing example
        if let Err(e) = challenges::challenge_shadowing() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the shadowing example!");
        
        // Challenge 3: Fix constant declaration
        if let Err(e) = challenges::challenge_constants() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the constant declaration!");
        
        // Challenge 4: Fix complex shadowing
        if let Err(e) = challenges::challenge_complex_shadowing() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the complex shadowing example!");
        
        // Challenge 5: Fix boolean shadowing
        if let Err(e) = challenges::challenge_boolean_shadowing() {
            return Err(format!("Challenge 5 failed: {}", e));
        }
        println!("Successfully fixed the boolean shadowing example!");
        
        println!("All Variables challenges completed successfully!");
        Ok(())
    }
} 