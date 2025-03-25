// Result in Rust
//
// This file demonstrates the Result type for handling recoverable errors.

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;

fn main() {
    println!("Understanding Result in Rust!");
    
    //------------------------------------------------------
    // BASIC RESULT HANDLING
    //------------------------------------------------------
    println!("\n=== Basic Result Handling ===");
    
    // Opening a file that might not exist
    let file_result = File::open("nonexistent_file.txt");
    
    // Using match with Result
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Error opening file: {:?}", error),
    }
    
    //------------------------------------------------------
    // HANDLING DIFFERENT ERROR TYPES
    //------------------------------------------------------
    println!("\n=== Handling Different Error Types ===");
    
    // More specific error handling with nested match
    let result = File::open("nonexistent_file.txt");
    match result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found - could create it"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error: {:?}", error),
        },
    }
    
    //------------------------------------------------------
    // SHORTER ERROR HANDLING WITH CLOSURES
    //------------------------------------------------------
    println!("\n=== Shorter Error Handling with Closures ===");
    
    // Using closures for more concise error handling
    let _file = File::open("nonexistent_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File not found, creating it...");
            File::create("nonexistent_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    //------------------------------------------------------
    // RESULT FROM PARSING
    //------------------------------------------------------
    println!("\n=== Result from Parsing ===");
    
    // Parsing a string to a number returns a Result
    let number_result: Result<i32, ParseIntError> = "42".parse();
    match number_result {
        Ok(number) => println!("Parsed number: {}", number),
        Err(error) => println!("Failed to parse: {:?}", error),
    }
    
    // Trying to parse an invalid number
    let invalid_number = "not_a_number".parse::<i32>();
    match invalid_number {
        Ok(number) => println!("Parsed number: {}", number),
        Err(error) => println!("Failed to parse invalid input: {:?}", error),
    }
    
    //------------------------------------------------------
    // HANDLING RESULTS WITH IF LET
    //------------------------------------------------------
    println!("\n=== Handling Results with if let ===");
    
    // Using if let for success case
    if let Ok(number) = "42".parse::<i32>() {
        println!("The number is: {}", number);
    }
    
    // Using if let for error case
    if let Err(error) = "not_a_number".parse::<i32>() {
        println!("Error parsing: {:?}", error);
    }
    
    //------------------------------------------------------
    // RETURNING RESULTS FROM FUNCTIONS
    //------------------------------------------------------
    println!("\n=== Returning Results from Functions ===");
    
    // Call our function and handle the result
    match read_from_file("nonexistent_file.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {:?}", error),
    }
}

// A function that returns a Result
fn read_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}

// The same function using the ? operator
fn read_from_file_concise(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Division with Result
//
// Implement the safe_divide function that takes two f64 values and returns a Result.
// It should handle division by zero by returning an appropriate error.

#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    // TODO: Implement this function
    // If b is zero, return an error; otherwise return a/b
    
    Ok(0.0) // Replace this placeholder
}

// Exercise 2: Parse Configuration Value
//
// Complete the parse_config_value function that takes a string and tries to parse it
// as a configuration value. The function should:
// - Return Ok(0) if the string is "zero" or "0"
// - Return Ok(1) if the string is "one" or "1"
// - Return Ok(2) if the string is "two" or "2"
// - Return an Err with a descriptive message for any other input

fn parse_config_value(value: &str) -> Result<i32, String> {
    // TODO: Implement this function
    
    Ok(0) // Replace this placeholder
}

// Exercise 3: Chain Multiple Operations
//
// Implement the calculate function that performs a series of operations,
// each of which might fail. The function should:
// 1. Parse the input string as an i32
// 2. Add 10 to the number
// 3. Convert the result to a string
// 4. Return the string

fn calculate(input: &str) -> Result<String, String> {
    // TODO: Implement this function
    // Use map, and_then, or other combinators to chain operations
    
    Ok(String::new()) // Replace this placeholder
} 