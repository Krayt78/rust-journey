// Unwrap and Expect in Rust
//
// This file demonstrates the unwrap and expect methods for
// quickly extracting values from Result and Option types.

use std::fs::File;
use std::collections::HashMap;

fn main() {
    println!("Understanding Unwrap and Expect in Rust!");
    
    //------------------------------------------------------
    // UNWRAP WITH RESULT
    //------------------------------------------------------
    println!("\n=== Unwrap with Result ===");
    
    // Successful unwrap
    let s = "42";
    let num = s.parse::<i32>().unwrap();
    println!("Parsed number: {}", num);
    
    // Uncomment to see unwrap panic on error:
    // let s = "not a number";
    // let num = s.parse::<i32>().unwrap(); // This will panic!
    println!("Parsing with unwrap succeeded");
    
    //------------------------------------------------------
    // EXPECT WITH RESULT
    //------------------------------------------------------
    println!("\n=== Expect with Result ===");
    
    // Successful expect
    let s = "42";
    let num = s.parse::<i32>().expect("Failed to parse number");
    println!("Parsed number with expect: {}", num);
    
    // Uncomment to see expect panic with custom message:
    // let s = "not a number";
    // let num = s.parse::<i32>().expect("Failed to parse number"); // Panic with custom message
    println!("Parsing with expect succeeded");
    
    //------------------------------------------------------
    // UNWRAP WITH OPTION
    //------------------------------------------------------
    println!("\n=== Unwrap with Option ===");
    
    // Successful Option unwrap
    let v = vec![1, 2, 3];
    let first = v.first().unwrap();
    println!("First element: {}", first);
    
    // Uncomment to see unwrap panic on None:
    // let v: Vec<i32> = vec![];
    // let first = v.first().unwrap(); // This will panic!
    println!("Vector unwrap succeeded");
    
    //------------------------------------------------------
    // EXPECT WITH OPTION
    //------------------------------------------------------
    println!("\n=== Expect with Option ===");
    
    // Using expect with Option
    let users = HashMap::from([
        (1, "Alice"),
        (2, "Bob"),
        (3, "Charlie"),
    ]);
    
    let user1 = users.get(&1).expect("User 1 not found");
    println!("User 1: {}", user1);
    
    // Uncomment to see expect panic with custom message:
    // let nonexistent = users.get(&999).expect("User 999 not found"); // Panic with custom message
    println!("HashMap lookup with expect succeeded");
    
    //------------------------------------------------------
    // APPROPRIATE USES OF UNWRAP/EXPECT
    //------------------------------------------------------
    println!("\n=== Appropriate Uses of Unwrap/Expect ===");
    
    // 1. When failure is impossible due to previous checks
    let numbers = vec![1, 2, 3];
    if !numbers.is_empty() {
        // Safe because we just checked that numbers is not empty
        let first = numbers.first().unwrap();
        println!("First number (safe unwrap): {}", first);
    }
    
    // 2. When you want to convert a recoverable error to unrecoverable
    //    for programming errors
    let config_value = match get_config_value("valid_key") {
        Some(value) => value,
        None => {
            // This is a programming error, invalid config key should never happen
            panic!("Missing required config key");
        }
    };
    println!("Config value: {}", config_value);
    
    // 3. In test code (would be in a #[test] function)
    let result = simulate_test_function();
    // In tests, failing assertions should fail the test
    assert_eq!(result.unwrap(), 42);
    println!("Test passed");
    
    //------------------------------------------------------
    // SAFER ALTERNATIVES
    //------------------------------------------------------
    println!("\n=== Safer Alternatives ===");
    
    // 1. Pattern matching
    let parse_result = "42".parse::<i32>();
    match parse_result {
        Ok(num) => println!("Parsed with match: {}", num),
        Err(e) => println!("Failed to parse with match: {}", e),
    }
    
    // 2. unwrap_or for default value
    let invalid_number = "not a number".parse::<i32>().unwrap_or(0);
    println!("Invalid number with default: {}", invalid_number);
    
    // 3. unwrap_or_else for computed default value
    let id_str = "not a number";
    let id = id_str.parse::<i32>().unwrap_or_else(|_| {
        println!("Computing default ID for invalid input: {}", id_str);
        generate_default_id()
    });
    println!("ID: {}", id);
    
    // 4. if let for only handling the success case
    if let Ok(num) = "42".parse::<i32>() {
        println!("Successfully parsed with if let: {}", num);
    }
    
    // 5. Calling separate functions for clarity
    process_option_value(Some(42));
    process_option_value(None);
}

// Helper function to simulate a config lookup
fn get_config_value(key: &str) -> Option<i32> {
    let config = HashMap::from([
        ("valid_key", 42),
        ("another_key", 100),
    ]);
    
    config.get(key).copied()
}

// Helper function to simulate a test
fn simulate_test_function() -> Result<i32, String> {
    Ok(42)
}

// Helper function to generate a default ID
fn generate_default_id() -> i32 {
    // In a real application, this might access a database or use some algorithm
    999
}

// Separate function to process an Option value
fn process_option_value(value: Option<i32>) {
    match value {
        Some(x) => println!("Got value: {}", x),
        None => println!("No value provided"),
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Safe Unwrapping
//
// Implement the get_element function to safely get the element at the 
// specified index from a vector, using unwrap appropriately.
// The function should return None if the index is out of bounds.

fn get_element(vec: &[i32], index: usize) -> Option<i32> {
    // TODO: Implement this function
    // Check if the index is valid first, then unwrap safely
    
    None // Replace this placeholder
}

// Exercise 2: Default Value Provider
//
// Improve the parse_age function to:
// 1. Try to parse the age string as an i32
// 2. If parsing fails, return a default age of 18
// 3. If the parsed age is negative, return 0
// 4. If the parsed age is over 120, return 120

fn parse_age(age_str: &str) -> i32 {
    // TODO: Implement this function
    // Use unwrap_or or unwrap_or_else to provide defaults
    
    0 // Replace this placeholder
}

// Exercise 3: Expect with Context
//
// Complete the function to load a user from a "database".
// If the user doesn't exist, use expect with a message that includes the user ID.

struct User {
    id: u32,
    name: String,
}

fn get_user_from_db(user_id: u32) -> Option<User> {
    let users = HashMap::from([
        (1, User { id: 1, name: String::from("Alice") }),
        (2, User { id: 2, name: String::from("Bob") }),
    ]);
    
    users.get(&user_id).cloned()
}

fn load_user(user_id: u32) -> User {
    // TODO: Implement this function
    // Use expect with a helpful message that includes the user_id
    
    User { id: 0, name: String::from("") } // Replace this placeholder
} 