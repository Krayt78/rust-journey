// Panic in Rust
//
// This file demonstrates panic for handling unrecoverable errors.

fn main() {
    println!("Understanding Panic in Rust!");
    
    // Uncomment each section to see different panic examples
    // (But only one at a time, since panics terminate the program!)

    //------------------------------------------------------
    // EXPLICIT PANIC
    //------------------------------------------------------
    println!("\n=== Explicit Panic ===");
    
    // Basic panic with a message
    // Uncomment to see the panic:
    // panic!("This is a panic!");
    
    // Panic with formatted message
    // Uncomment to see the panic:
    // let error_code = 42;
    // panic!("Error code {}: Application crashed", error_code);

    //------------------------------------------------------
    // IMPLICIT PANICS
    //------------------------------------------------------
    println!("\n=== Implicit Panics ===");
    
    // Array bounds
    let numbers = [1, 2, 3, 4, 5];
    println!("First element: {}", numbers[0]);
    
    // Uncomment to see an out-of-bounds panic:
    // let out_of_bounds = numbers[10]; // This will panic!
    
    // Integer division by zero
    // Uncomment to see a division by zero panic:
    // let result = 10 / 0; // This will panic!
    
    //------------------------------------------------------
    // ASSERTIONS
    //------------------------------------------------------
    println!("\n=== Assertions ===");
    
    // assert! macro (panics if condition is false)
    let x = 10;
    assert!(x > 5, "x must be greater than 5");
    
    // assert_eq! macro (panics if values aren't equal)
    let a = 5;
    let b = 5;
    assert_eq!(a, b, "a must equal b");
    
    // assert_ne! macro (panics if values are equal)
    let c = 10;
    let d = 20;
    assert_ne!(c, d, "c must not equal d");
    
    // Uncomment to see an assertion panic:
    // assert!(x > 50, "x must be greater than 50");
    
    //------------------------------------------------------
    // UNWRAP AND EXPECT
    //------------------------------------------------------
    println!("\n=== Unwrap and Expect ===");
    
    // Successful unwrap
    let s = "42";
    let num = s.parse::<i32>().unwrap();
    println!("Parsed number: {}", num);
    
    // Uncomment to see unwrap panic on parse error:
    // let s = "not a number";
    // let num = s.parse::<i32>().unwrap(); // This will panic!
    
    // expect gives a better error message
    // Uncomment to see expect panic:
    // let s = "not a number";
    // let num = s.parse::<i32>().expect("Failed to parse number"); // This will panic with custom message!
    
    //------------------------------------------------------
    // UNREACHABLE CODE
    //------------------------------------------------------
    println!("\n=== Unreachable Code ===");
    
    let dice_roll = 6;
    match dice_roll {
        1..=6 => println!("Valid dice roll: {}", dice_roll),
        // Uncomment to see unreachable panic:
        // _ => unreachable!("Invalid dice roll: {}", dice_roll),
    }

    println!("No panics occurred!");
}

//------------------------------------------------------
// ADDITIONAL EXAMPLES
//------------------------------------------------------

// Causing a panic in a called function
fn might_panic(value: i32) {
    if value < 0 {
        panic!("Value must be non-negative");
    }
    println!("Value is valid: {}", value);
}

// This function demonstrates a common pattern: avoiding panic by returning Result
fn avoid_panic(value: i32) -> Result<i32, String> {
    if value < 0 {
        Err(format!("Value must be non-negative, got {}", value))
    } else {
        Ok(value)
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Debug Assertion
// 
// Complete the function to perform a debug-only assertion that the input
// slice has at least 3 elements. Use the debug_assert! macro so the check
// is only performed in debug builds.

fn process_at_least_three(items: &[i32]) -> i32 {
    // TODO: Add a debug assertion to check that items has at least 3 elements
    
    items[0] + items[1] + items[2]
}

// Exercise 2: Custom Failure Condition
//
// Complete the function to check if a username is valid according to these rules:
// - Must be at least 3 characters long
// - Must not be longer than 20 characters
// - Must contain only alphanumeric characters
// 
// If the username is invalid, panic with an appropriate message.

fn validate_username(username: &str) {
    // TODO: Check the username rules and panic if invalid
}