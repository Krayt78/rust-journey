// Error Handling in Rust
//
// This file demonstrates the various error handling mechanisms in Rust,
// including panics, Result, Option, and custom error types.

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;
use std::collections::HashMap;

fn main() {
    println!("Understanding Error Handling in Rust!");
    
    // Each example is separated into its own function
    demonstrate_panics();
    demonstrate_result();
    demonstrate_unwrap_and_expect();
    demonstrate_error_propagation();
    demonstrate_option();
    demonstrate_combinators();
    demonstrate_custom_errors();
}

//------------------------------------------------------
// PANIC: UNRECOVERABLE ERRORS
//------------------------------------------------------
fn demonstrate_panics() {
    println!("\n=== Panic for Unrecoverable Errors ===");
    
    // Uncomment the following line to see a panic:
    // panic!("This is a panic!");
    
    // Panic caused by invalid array access:
    let v = vec![1, 2, 3];
    // Uncomment the following line to see a panic:
    // let item = v[99]; // This will panic!
    
    // Using assert! (panics if condition is false)
    let x = 10;
    assert!(x > 5, "x must be greater than 5");
    
    // Using assert_eq! (panics if values aren't equal)
    let a = 5;
    let b = 5;
    assert_eq!(a, b, "a must equal b");
    
    println!("No panics occurred!");
}

//------------------------------------------------------
// RESULT: RECOVERABLE ERRORS
//------------------------------------------------------
fn demonstrate_result() {
    println!("\n=== Result for Recoverable Errors ===");
    
    // Opening a file that might not exist
    let file_result = File::open("nonexistent_file.txt");
    
    // Using match with Result
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Error opening file: {:?}", error),
    }
    
    // More specific error handling
    let result = File::open("nonexistent_file.txt");
    match result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error: {:?}", error),
        },
    }
    
    // Shorthand with if let
    if let Err(error) = File::open("nonexistent_file.txt") {
        println!("Failed to open file: {:?}", error);
    }
}

//------------------------------------------------------
// USING UNWRAP AND EXPECT
//------------------------------------------------------
fn demonstrate_unwrap_and_expect() {
    println!("\n=== Unwrap and Expect ===");
    
    // unwrap() - extracts the value or panics if Result is Err
    // Uncomment to see a panic:
    // let file = File::open("nonexistent_file.txt").unwrap();
    
    // expect() - like unwrap, but with a custom panic message
    // Uncomment to see a panic:
    // let file = File::open("nonexistent_file.txt").expect("Failed to open file");
    
    // Successful unwrap
    let s = "42";
    let num = s.parse::<i32>().unwrap();
    println!("Parsed number: {}", num);
}

//------------------------------------------------------
// PROPAGATING ERRORS
//------------------------------------------------------
fn demonstrate_error_propagation() {
    println!("\n=== Propagating Errors ===");
    
    // Demonstrate a function that returns a Result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error reading username: {:?}", error),
    }
    
    // Using the ? operator (in a different function)
    match read_username_from_file_concise() {
        Ok(username) => println!("Username (concise): {}", username),
        Err(error) => println!("Error reading username (concise): {:?}", error),
    }
    
    // Show the other versions (without calling them)
    println!("We also have even more concise versions:");
    println!("1. With method chaining: read_username_from_file_one_line()");
    println!("2. Using std library: read_username_from_file_shortest()");
}

//------------------------------------------------------
// OPTION: HANDLING ABSENCE OF VALUE
//------------------------------------------------------
fn demonstrate_option() {
    println!("\n=== Option for Absent Values ===");
    
    // A function that returns Option
    let user_id = 42;
    match get_user(user_id) {
        Some(name) => println!("User {} found: {}", user_id, name),
        None => println!("User {} not found", user_id),
    }
    
    // Using if let with Option
    if let Some(name) = get_user(999) {
        println!("User 999 found: {}", name);
    } else {
        println!("User 999 not found");
    }
    
    // Option combinators
    let user_id = 42;
    let display_name = get_user(user_id)
        .map(|name| name.to_uppercase())
        .unwrap_or_else(|| format!("USER_{}", user_id));
    println!("Display name: {}", display_name);
}

//------------------------------------------------------
// COMBINATORS
//------------------------------------------------------
fn demonstrate_combinators() {
    println!("\n=== Result Combinators ===");
    
    // Convert a string to an integer
    let number_result = "42".parse::<i32>();
    
    // Using map to transform the Ok value
    let incremented = number_result.map(|num| num + 1);
    println!("Incremented: {:?}", incremented);
    
    // Using and_then for chaining operations
    let number_result = "42".parse::<i32>();
    let squared = number_result.and_then(|num| Ok(num * num));
    println!("Squared: {:?}", squared);
    
    // Using or_else to handle errors
    let number_result = "not a number".parse::<i32>();
    let fallback = number_result.or_else(|_| Ok(0));
    println!("Fallback: {:?}", fallback);
    
    // Using unwrap_or to provide a default value
    let number = "not a number".parse::<i32>().unwrap_or(0);
    println!("Number or default: {}", number);
}

//------------------------------------------------------
// CUSTOM ERROR TYPES
//------------------------------------------------------
fn demonstrate_custom_errors() {
    println!("\n=== Custom Error Types ===");
    
    // Using the custom error type
    match read_config_file("config.txt") {
        Ok(config) => println!("Config: {:?}", config),
        Err(error) => println!("Config error: {:?}", error),
    }
    
    // Show a successful case
    match read_config_file("valid_config.txt") {
        Ok(config) => println!("Valid config: {:?}", config),
        Err(error) => println!("Config error: {:?}", error),
    }
}

// A function that returns a Result
fn read_username_from_file() -> Result<String, io::Error> {
    // Attempt to open a file
    let file_result = File::open("username.txt");
    
    // Handle the Result returned by open
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    // Read the file contents
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// The same function using the ? operator
fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Even more concise with method chaining
fn read_username_from_file_one_line() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Using the standard library function
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

// A function that returns an Option
fn get_user(id: u32) -> Option<String> {
    let users = HashMap::from([
        (42, "Alice"),
        (43, "Bob"),
        (44, "Charlie"),
    ]);
    
    // Convert to an Option
    users.get(&id).map(|&name| name.to_string())
}

// Custom error type
#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError(ParseIntError),
    MissingField(String),
    InvalidValue(String),
}

// Implement conversions from specific error types
impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(error: ParseIntError) -> Self {
        ConfigError::ParseError(error)
    }
}

// Function that uses the custom error type
fn read_config_file(filename: &str) -> Result<HashMap<String, String>, ConfigError> {
    // This would normally read from a file, but for this example:
    // we'll just simulate an error
    
    if filename == "config.txt" {
        Err(ConfigError::MissingField("database_url".to_string()))
    } else {
        // This would normally have actual config values
        let mut config = HashMap::new();
        config.insert("database_url".to_string(), "postgres://localhost".to_string());
        config.insert("port".to_string(), "8080".to_string());
        Ok(config)
    }
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Parse a Configuration
//
// Implement the function parse_config that takes a string of comma-separated key-value pairs
// (e.g., "name=Alice,age=30,active=true") and returns a Result with a HashMap.
// If a value is missing or malformed, return an appropriate error.

#[derive(Debug)]
enum ParseConfigError {
    MissingEquals,
    EmptyKey,
    EmptyValue,
    DuplicateKey(String),
}

fn parse_config(config_str: &str) -> Result<HashMap<String, String>, ParseConfigError> {
    // TODO: Implement this function.
    // Parse a configuration string in the format "key1=value1,key2=value2"
    // Return a HashMap with the keys and values.
    // Handle appropriate errors if:
    // - A pair doesn't contain an equals sign
    // - A key is empty
    // - A value is empty
    // - A key appears more than once
    
    Ok(HashMap::new()) // Replace this placeholder
}

// Challenge 2: Safely Perform Division
//
// Implement the function safe_divide that takes two f64 values and returns a Result.
// It should handle division by zero and return an appropriate error.

#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    // TODO: Implement this function.
    // Return the result of a/b, or an error if b is zero.
    
    Ok(0.0) // Replace this placeholder
}

// Challenge 3: Chain of Calculations
//
// Implement the function calculate that takes a starting value and performs
// a series of operations using the ? operator to propagate errors.
// The function should:
// 1. Take a starting value
// 2. Add 10
// 3. Divide by another input
// 4. Subtract 5
// Return the final result or an appropriate error.

fn calculate(start: f64, divisor: f64) -> Result<f64, MathError> {
    // TODO: Implement this function.
    // Perform the sequence of operations described above using the ? operator.
    
    Ok(0.0) // Replace this placeholder
}

// Challenge 4: Find a User
//
// Implement the function find_user_by_email that takes an email address
// and returns information about the user, including the result of a second
// function that fetches additional user data.
// Handle errors appropriately, including when the user is not found or
// when fetching user data fails.

#[derive(Debug)]
struct UserData {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
enum UserError {
    NotFound,
    InactiveUser,
    DatabaseError(String),
}

fn find_user_by_email(email: &str) -> Result<UserData, UserError> {
    // TODO: Implement this function
    // Simulate finding a user in a database.
    // If the email is "alice@example.com", return a valid UserData.
    // If the email is "inactive@example.com", return a UserError::InactiveUser.
    // For any other email, return UserError::NotFound.
    
    Err(UserError::NotFound) // Replace this placeholder
}

// Simulate fetching additional data
fn get_user_posts(user_id: u32) -> Result<Vec<String>, UserError> {
    // This is just a simulation, in reality this would access a database
    if user_id == 1 {
        Ok(vec![
            "First post".to_string(),
            "Second post".to_string(),
        ])
    } else if user_id == 2 {
        Err(UserError::InactiveUser)
    } else {
        Err(UserError::DatabaseError("Connection failed".to_string()))
    }
}

// Implement this function to use both find_user_by_email and get_user_posts
fn get_user_with_posts(email: &str) -> Result<(UserData, Vec<String>), UserError> {
    // TODO: Implement this function
    // Find the user by email, then get their posts.
    // Return both the user data and posts if successful.
    // Use the ? operator for error propagation.
    
    Err(UserError::NotFound) // Replace this placeholder
}

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_config() {
        // Valid configuration
        let config = parse_config("name=Alice,age=30,active=true").unwrap();
        assert_eq!(config.get("name"), Some(&"Alice".to_string()));
        assert_eq!(config.get("age"), Some(&"30".to_string()));
        assert_eq!(config.get("active"), Some(&"true".to_string()));
        
        // Missing equals sign
        match parse_config("name=Alice,age") {
            Err(ParseConfigError::MissingEquals) => (),
            _ => panic!("Expected MissingEquals error"),
        }
        
        // Empty key
        match parse_config("=value") {
            Err(ParseConfigError::EmptyKey) => (),
            _ => panic!("Expected EmptyKey error"),
        }
        
        // Empty value
        match parse_config("key=") {
            Err(ParseConfigError::EmptyValue) => (),
            _ => panic!("Expected EmptyValue error"),
        }
        
        // Duplicate key
        match parse_config("name=Alice,name=Bob") {
            Err(ParseConfigError::DuplicateKey(key)) => assert_eq!(key, "name"),
            _ => panic!("Expected DuplicateKey error"),
        }
    }
    
    #[test]
    fn test_safe_divide() {
        // Valid division
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        
        // Division by zero
        match safe_divide(10.0, 0.0) {
            Err(MathError::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }
    
    #[test]
    fn test_calculate() {
        // Valid calculation
        assert_eq!(calculate(10.0, 5.0).unwrap(), 3.0); // (10 + 10) / 5 - 5 = 4 - 5 = -1
        
        // Division by zero
        match calculate(10.0, 0.0) {
            Err(MathError::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }
    
    #[test]
    fn test_find_user_by_email() {
        // Valid user
        let user = find_user_by_email("alice@example.com").unwrap();
        assert_eq!(user.name, "Alice");
        
        // Inactive user
        match find_user_by_email("inactive@example.com") {
            Err(UserError::InactiveUser) => (),
            _ => panic!("Expected InactiveUser error"),
        }
        
        // User not found
        match find_user_by_email("nonexistent@example.com") {
            Err(UserError::NotFound) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
    
    #[test]
    fn test_get_user_with_posts() {
        // Valid user with posts
        let (user, posts) = get_user_with_posts("alice@example.com").unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(posts.len(), 2);
        
        // Inactive user
        match get_user_with_posts("inactive@example.com") {
            Err(UserError::InactiveUser) => (),
            _ => panic!("Expected InactiveUser error"),
        }
        
        // User not found
        match get_user_with_posts("nonexistent@example.com") {
            Err(UserError::NotFound) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
} 