// Borrowing in Rust
//
// This program demonstrates Rust's borrowing system, including:
// - References and borrowing
// - Mutable vs. immutable references
// - The borrowing rules
// - Preventing dangling references
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    println!("Exploring Borrowing in Rust!");
    
    //------------------------------------------------------
    // REFERENCES AND BORROWING
    //------------------------------------------------------
    println!("\n--- References and Borrowing ---");
    
    // Instead of taking ownership, we can pass a reference
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // Pass a reference to s1
    
    println!("The length of '{}' is {}.", s1, len);
    // s1 is still valid here because calculate_length only borrowed it
    
    //------------------------------------------------------
    // MUTABLE REFERENCES
    //------------------------------------------------------
    println!("\n--- Mutable References ---");
    
    let mut s = String::from("hello");
    println!("Before change: {}", s);
    
    change(&mut s); // Pass a mutable reference
    
    println!("After change: {}", s);
    
    //------------------------------------------------------
    // BORROWING RULES: RESTRICTION 1
    //------------------------------------------------------
    println!("\n--- Borrowing Rules: One Mutable Reference ---");
    
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time
    
    println!("Mutable reference: {}", r1);
    
    // Once r1 is no longer used, we can create another mutable reference
    let r2 = &mut s;
    println!("Another mutable reference: {}", r2);
    
    //------------------------------------------------------
    // BORROWING RULES: RESTRICTION 2
    //------------------------------------------------------
    println!("\n--- Borrowing Rules: Immutable and Mutable References ---");
    
    let mut s = String::from("hello");
    
    let r1 = &s; // Immutable reference
    let r2 = &s; // Another immutable reference
    println!("Two immutable references: {} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // This is fine because r1 and r2 are no longer used
    println!("Mutable reference: {}", r3);
    
    //------------------------------------------------------
    // BORROWING RULES: SCOPE-BASED
    //------------------------------------------------------
    println!("\n--- Borrowing Rules: Non-Lexical Lifetimes ---");
    
    let mut s = String::from("hello");
    
    let r1 = &s; // Immutable reference
    let r2 = &s; // Another immutable reference
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // This is fine
    println!("r3: {}", r3);
    
    //------------------------------------------------------
    // PREVENTING DANGLING REFERENCES
    //------------------------------------------------------
    println!("\n--- Preventing Dangling References ---");
    
    // This would cause a compile error:
    // let reference_to_nothing = dangle();
    
    let s = no_dangle();
    println!("Safe return: {}", s);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope, but since it doesn't have ownership, nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This function would cause a compile error:
// fn dangle() -> &String { // Error: would return a reference to a dropped value
//     let s = String::from("hello");
//     &s
// } // s goes out of scope and is dropped, so the reference would be invalid

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return the String directly, transferring ownership
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix this code to use references properly
    pub fn challenge_references() -> Result<(), String> {
        let s = String::from("hello");
        
        // This function should take a reference to s, not ownership
        let length = calculate_length(s);
        
        // We need to use s here, but it's been moved
        if s != "hello" || length != 5 {
            return Err(format!("Values are incorrect. s: {:?}, length: {}", s, length));
        }
        
        Ok(())
    }
    
    // This function may need to be modified
    fn calculate_length(s: String) -> usize {
        s.len()
    }
    
    // TODO: Fix this code to handle mutable references correctly
    pub fn challenge_mutable_references() -> Result<(), String> {
        let s = String::from("hello");
        
        // This function should modify the string to "hello, world"
        add_world(&s);
        
        if s != "hello, world" {
            return Err(format!("Value is incorrect: {:?}", s));
        }
        
        Ok(())
    }
    
    // This function may need to be modified
    fn add_world(s: &String) {
        s.push_str(", world");
    }
    
    // TODO: Fix this code to handle multiple references correctly
    pub fn challenge_multiple_references() -> Result<(), String> {
        let mut s = String::from("hello");
        
        // This code has multiple borrowing issues
        let r1 = &s;
        let r2 = &mut s;
        let r3 = &s;
        
        println!("{}, {}, and {}", r1, r2, r3);
        
        Ok(())
    }
    
    // TODO: Fix this function to avoid returning a dangling reference
    pub fn challenge_dangling_reference() -> Result<(), String> {
        let reference;
        
        {
            let s = String::from("hello");
            reference = &s; // This creates a dangling reference
        } // s goes out of scope here
        
        if reference != "hello" {
            return Err("Reference should point to 'hello'".to_string());
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
        println!("\nRunning Borrowing challenges...");
        
        // Challenge 1: Fix references
        if let Err(e) = challenges::challenge_references() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the references issue!");
        
        // Challenge 2: Fix mutable references
        if let Err(e) = challenges::challenge_mutable_references() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the mutable references issue!");
        
        // Challenge 3: Fix multiple references
        if let Err(e) = challenges::challenge_multiple_references() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the multiple references issue!");
        
        // Challenge 4: Fix dangling reference
        if let Err(e) = challenges::challenge_dangling_reference() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the dangling reference issue!");
        
        println!("All Borrowing challenges completed successfully!");
        Ok(())
    }
} 