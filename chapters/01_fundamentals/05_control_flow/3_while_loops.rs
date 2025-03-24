// While Loops in Rust
//
// This program demonstrates how while loops work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while

fn main() {
    println!("Exploring While Loops in Rust!");
    
    //------------------------------------------------------
    // BASIC WHILE LOOP
    //------------------------------------------------------
    println!("\n--- Basic While Loop ---");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
    
    //------------------------------------------------------
    // WHILE LOOP FOR COLLECTION PROCESSING
    //------------------------------------------------------
    println!("\n--- While Loop for Processing Collections ---");
    
    let mut index = 0;
    let array = [10, 20, 30, 40, 50];
    
    while index < array.len() {
        println!("The value at index {} is: {}", index, array[index]);
        index += 1;
    }
    
    //------------------------------------------------------
    // WHILE LOOP WITH CONDITION CHECK
    //------------------------------------------------------
    println!("\n--- While Loop with Complex Condition ---");
    
    let mut count = 0;
    let secret_number = 7;
    
    while count < 10 && count != secret_number {
        println!("Count is: {}", count);
        count += 1;
    }
    
    if count == secret_number {
        println!("Found the secret number: {}", secret_number);
    } else {
        println!("Reached the maximum count without finding the secret");
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the while loop to print numbers from 1 to 3
    pub fn challenge_while_loop() -> Result<(), String> {
        let mut counter = 3;
        let mut output = String::new();
        
        // This while loop should count from 1 to 3, but it's counting down
        // Fix it to count up from 1 to 3
        while counter > 0 {
            output.push_str(&format!("{}", counter));
            counter -= 1;
        }
        
        if output != "123" {
            return Err(format!("Output should be '123', but got '{}'", output));
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
        println!("\nRunning While Loop challenges...");
        
        // Challenge: Fix while loop
        if let Err(e) = challenges::challenge_while_loop() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the while loop!");
        
        println!("All While Loop challenges completed successfully!");
        Ok(())
    }
} 