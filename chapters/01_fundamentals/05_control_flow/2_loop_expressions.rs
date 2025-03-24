// Loop Expressions in Rust
//
// This program demonstrates how loop expressions work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops

fn main() {
    println!("Exploring Loop Expressions in Rust!");
    
    //------------------------------------------------------
    // BASIC LOOP
    //------------------------------------------------------
    println!("\n--- Basic Loop ---");
    
    // A simple loop that breaks after one iteration
    let mut counter = 0;
    loop {
        println!("Inside the loop!");
        counter += 1;
        if counter == 1 {
            break;
        }
    }
    
    //------------------------------------------------------
    // LOOP WITH RETURN VALUE
    //------------------------------------------------------
    println!("\n--- Loop with Return Value ---");
    
    counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // Return a value from the loop
        }
    };
    
    println!("The result of the loop is: {}", result);
    
    //------------------------------------------------------
    // NESTED LOOPS WITH LABELS
    //------------------------------------------------------
    println!("\n--- Nested Loops with Labels ---");
    
    let mut count = 0;
    'outer: loop {
        println!("Outer loop count: {}", count);
        let mut inner_count = 0;
        
        'inner: loop {
            println!("  Inner loop count: {}", inner_count);
            inner_count += 1;
            
            if inner_count >= 2 {
                println!("  Breaking inner loop");
                break 'inner;
            }
        }
        
        count += 1;
        if count >= 2 {
            println!("Breaking outer loop");
            break 'outer;
        }
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the loop to correctly count up to 5
    pub fn challenge_loop() -> Result<(), String> {
        let mut counter = 0;
        let result;
        
        // This loop should count up to 5 and return counter * 2
        // but there's an issue with the break condition
        result = loop {
            counter += 1;
            
            if counter == 3 {
                break counter * 2;
            }
        };
        
        if result != 10 {
            return Err(format!("The result should be 10, but got {}", result));
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
        println!("\nRunning Loop Expression challenges...");
        
        // Challenge: Fix loop
        if let Err(e) = challenges::challenge_loop() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the loop!");
        
        println!("All Loop Expression challenges completed successfully!");
        Ok(())
    }
} 