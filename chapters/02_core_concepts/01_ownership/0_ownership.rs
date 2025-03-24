// Ownership in Rust
//
// This program demonstrates Rust's ownership system, including:
// - Ownership rules
// - Move semantics
// - Copy types
// - Functions and ownership
// - Return values and scope
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    println!("Exploring Ownership in Rust!");
    
    //------------------------------------------------------
    // BASIC OWNERSHIP RULES
    //------------------------------------------------------
    println!("\n--- Basic Ownership Rules ---");
    
    // 1. Each value has a variable that's its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped
    
    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("String inside scope: {}", s);
        // do stuff with s
    } // scope is now over, and s is no longer valid - Rust calls the `drop` function automatically
    
    // This would cause a compile error:
    // println!("String after scope: {}", s);
    
    //------------------------------------------------------
    // MOVE SEMANTICS
    //------------------------------------------------------
    println!("\n--- Move Semantics ---");
    
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    
    let s2 = s1; // s1's value is moved to s2
    println!("s2: {}", s2);
    
    // This would cause a compile error since s1's value has been moved:
    // println!("s1 after move: {}", s1);
    
    //------------------------------------------------------
    // COPY TYPES
    //------------------------------------------------------
    println!("\n--- Copy Types ---");
    
    // Simple types like integers are copied, not moved
    let x = 5;
    let y = x; // x is still valid because integers are Copy types
    
    println!("x: {}, y: {}", x, y); // Both are valid
    
    // Types that implement the Copy trait are:
    // - All integer types (i32, u64, etc.)
    // - Boolean type (bool)
    // - Floating point types (f32, f64)
    // - Character type (char)
    // - Tuples, but only if they contain types that are also Copy
    
    //------------------------------------------------------
    // FUNCTIONS AND OWNERSHIP
    //------------------------------------------------------
    println!("\n--- Functions and Ownership ---");
    
    let s = String::from("hello");
    println!("Before function call: {}", s);
    
    takes_ownership(s); // s's value moves into the function
    
    // This would cause a compile error:
    // println!("After function call: {}", s);
    
    let x = 5;
    println!("Before function call: {}", x);
    
    makes_copy(x); // x would move into the function, but i32 is Copy
    
    println!("After function call: {}", x); // Still valid
    
    //------------------------------------------------------
    // RETURN VALUES AND SCOPE
    //------------------------------------------------------
    println!("\n--- Return Values and Scope ---");
    
    let s1 = gives_ownership(); // get a String from the function
    println!("Got ownership of: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function, which returns a value that's moved to s3
    
    println!("Got back: {}", s3);
    
    // This would cause a compile error since s2 was moved:
    // println!("s2 after move: {}", s2);
    
    //------------------------------------------------------
    // MULTIPLE RETURN VALUES
    //------------------------------------------------------
    println!("\n--- Multiple Return Values ---");
    
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);
    
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("Function has: {}", some_string);
} // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Function has: {}", some_integer);
} // some_integer goes out of scope, but nothing special happens

fn gives_ownership() -> String { // will move its return value
    let some_string = String::from("hello");
    some_string // returned and ownership is moved out
}

fn takes_and_gives_back(a_string: String) -> String { // takes and returns a String
    a_string // returned and ownership is moved out
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // return multiple values as a tuple
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix this code to handle ownership correctly
    pub fn challenge_move_semantics() -> Result<(), String> {
        let original = String::from("hello");
        
        // This code is broken due to ownership issues
        let new = original;
        // We need to use both original and new strings here
        
        if original != "hello" || new != "hello" {
            return Err(format!("Values are incorrect. original: {:?}, new: {:?}", original, new));
        }
        
        Ok(())
    }
    
    // TODO: Fix this function to correctly return the string and its length
    pub fn challenge_return_ownership() -> Result<(), String> {
        let s = String::from("ownership");
        
        // This function should return both the string and its length
        let length = calculate_length(s);
        
        // We need s here, but it's been moved
        if s != "ownership" || length != 9 {
            return Err(format!("Values are incorrect. s: {:?}, length: {}", s, length));
        }
        
        Ok(())
    }
    
    // Helper function - this one may need to be modified
    fn calculate_length(s: String) -> usize {
        s.len()
    }
    
    // TODO: Fix ownership issues with function calls
    pub fn challenge_function_ownership() -> Result<(), String> {
        let message = String::from("important data");
        
        // The function should print the message without taking ownership
        process_message(message);
        
        // We need to use message again here
        if message != "important data" {
            return Err(format!("Value is incorrect: {:?}", message));
        }
        
        Ok(())
    }
    
    // This function may need to be modified
    fn process_message(m: String) {
        println!("Processing: {}", m);
    }
    
    // TODO: Fix this to use copy semantics correctly
    pub fn challenge_copy_types() -> Result<(), String> {
        let numbers = (10, 20, 30);
        
        // Should be able to use numbers after this call
        let sum = sum_tuple(numbers);
        
        if sum != 60 || numbers.0 != 10 {
            return Err(format!("Values are incorrect. sum: {}, numbers: {:?}", sum, numbers));
        }
        
        Ok(())
    }
    
    // This function may need to be modified
    fn sum_tuple(nums: (i32, i32, i32)) -> i32 {
        nums.0 + nums.1 + nums.2
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Ownership challenges...");
        
        // Challenge 1: Fix move semantics
        if let Err(e) = challenges::challenge_move_semantics() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the move semantics issue!");
        
        // Challenge 2: Fix returning ownership
        if let Err(e) = challenges::challenge_return_ownership() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the return ownership issue!");
        
        // Challenge 3: Fix function ownership
        if let Err(e) = challenges::challenge_function_ownership() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the function ownership issue!");
        
        // Challenge 4: Fix copy types
        if let Err(e) = challenges::challenge_copy_types() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the copy types issue!");
        
        println!("All Ownership challenges completed successfully!");
        Ok(())
    }
} 