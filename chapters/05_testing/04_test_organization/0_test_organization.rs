// Test Organization in Rust
//
// This file demonstrates techniques for organizing tests in Rust,
// including test modules, fixtures, and setup/teardown patterns.

use std::collections::HashMap;

fn main() {
    println!("Test Organization in Rust");
    println!("Run the tests with: cargo test");
    
    // Example usage of the code we'll be testing
    let mut user_db = UserDatabase::new();
    user_db.create_user("alice", 28, "alice@example.com");
    user_db.create_user("bob", 35, "bob@example.com");
    
    println!("Users in database: {}", user_db.count());
    
    match user_db.get_user("alice") {
        Some(user) => println!("Found user: {} ({}, {})", user.name, user.age, user.email),
        None => println!("User not found"),
    }
    
    println!("\nComplete the exercises at the end of this file.");
}

//------------------------------------------------------
// USER DATABASE EXAMPLE
//------------------------------------------------------

/// Represents a user in the system
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// A simple in-memory user database
pub struct UserDatabase {
    users: HashMap<String, User>,
}

impl UserDatabase {
    /// Creates a new, empty user database
    pub fn new() -> Self {
        UserDatabase {
            users: HashMap::new(),
        }
    }
    
    /// Creates a new user and adds them to the database
    pub fn create_user(&mut self, name: &str, age: u32, email: &str) -> Result<(), String> {
        // Check if user already exists
        if self.users.contains_key(name) {
            return Err(format!("User '{}' already exists", name));
        }
        
        // Validate email format (very basic validation)
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        // Add the user
        let user = User {
            name: name.to_string(),
            age,
            email: email.to_string(),
        };
        
        self.users.insert(name.to_string(), user);
        Ok(())
    }
    
    /// Retrieves a user by name
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }
    
    /// Updates a user's information
    pub fn update_user(&mut self, name: &str, age: u32, email: &str) -> Result<(), String> {
        // Check if user exists
        if !self.users.contains_key(name) {
            return Err(format!("User '{}' not found", name));
        }
        
        // Validate email format
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        // Update the user
        let user = User {
            name: name.to_string(),
            age,
            email: email.to_string(),
        };
        
        self.users.insert(name.to_string(), user);
        Ok(())
    }
    
    /// Deletes a user from the database
    pub fn delete_user(&mut self, name: &str) -> bool {
        self.users.remove(name).is_some()
    }
    
    /// Returns the number of users in the database
    pub fn count(&self) -> usize {
        self.users.len()
    }
    
    /// Lists all users in the database
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}

//------------------------------------------------------
// BASIC TEST ORGANIZATION
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests for user creation
    mod user_creation_tests {
        use super::*;
        
        #[test]
        fn test_create_valid_user() {
            let mut db = UserDatabase::new();
            let result = db.create_user("alice", 28, "alice@example.com");
            
            assert!(result.is_ok());
            assert_eq!(db.count(), 1);
            
            let user = db.get_user("alice").unwrap();
            assert_eq!(user.name, "alice");
            assert_eq!(user.age, 28);
            assert_eq!(user.email, "alice@example.com");
        }
        
        #[test]
        fn test_create_duplicate_user() {
            let mut db = UserDatabase::new();
            db.create_user("alice", 28, "alice@example.com").unwrap();
            
            let result = db.create_user("alice", 30, "alice2@example.com");
            assert!(result.is_err());
            assert_eq!(db.count(), 1);
        }
        
        #[test]
        fn test_create_user_invalid_email() {
            let mut db = UserDatabase::new();
            let result = db.create_user("alice", 28, "invalid-email");
            
            assert!(result.is_err());
            assert_eq!(db.count(), 0);
        }
    }
    
    // Tests for user retrieval
    mod user_retrieval_tests {
        use super::*;
        
        #[test]
        fn test_get_existing_user() {
            let mut db = UserDatabase::new();
            db.create_user("alice", 28, "alice@example.com").unwrap();
            
            let user = db.get_user("alice");
            assert!(user.is_some());
            
            let user = user.unwrap();
            assert_eq!(user.name, "alice");
            assert_eq!(user.age, 28);
            assert_eq!(user.email, "alice@example.com");
        }
        
        #[test]
        fn test_get_nonexistent_user() {
            let db = UserDatabase::new();
            let user = db.get_user("alice");
            assert!(user.is_none());
        }
    }
    
    // Tests for user updates
    mod user_update_tests {
        use super::*;
        
        #[test]
        fn test_update_existing_user() {
            let mut db = UserDatabase::new();
            db.create_user("alice", 28, "alice@example.com").unwrap();
            
            let result = db.update_user("alice", 29, "alice.updated@example.com");
            assert!(result.is_ok());
            
            let user = db.get_user("alice").unwrap();
            assert_eq!(user.age, 29);
            assert_eq!(user.email, "alice.updated@example.com");
        }
        
        #[test]
        fn test_update_nonexistent_user() {
            let mut db = UserDatabase::new();
            let result = db.update_user("alice", 28, "alice@example.com");
            assert!(result.is_err());
        }
        
        #[test]
        fn test_update_user_invalid_email() {
            let mut db = UserDatabase::new();
            db.create_user("alice", 28, "alice@example.com").unwrap();
            
            let result = db.update_user("alice", 29, "invalid-email");
            assert!(result.is_err());
            
            // Check that the user wasn't updated
            let user = db.get_user("alice").unwrap();
            assert_eq!(user.email, "alice@example.com");
        }
    }
    
    // Tests for user deletion
    mod user_deletion_tests {
        use super::*;
        
        #[test]
        fn test_delete_existing_user() {
            let mut db = UserDatabase::new();
            db.create_user("alice", 28, "alice@example.com").unwrap();
            assert_eq!(db.count(), 1);
            
            let result = db.delete_user("alice");
            assert!(result);
            assert_eq!(db.count(), 0);
            assert!(db.get_user("alice").is_none());
        }
        
        #[test]
        fn test_delete_nonexistent_user() {
            let mut db = UserDatabase::new();
            let result = db.delete_user("alice");
            assert!(!result);
        }
    }
}

//------------------------------------------------------
// TEST FIXTURES AND SETUP
//------------------------------------------------------

#[cfg(test)]
mod fixture_tests {
    use super::*;
    
    // Test fixture for a populated database
    fn setup_populated_db() -> UserDatabase {
        let mut db = UserDatabase::new();
        db.create_user("alice", 28, "alice@example.com").unwrap();
        db.create_user("bob", 35, "bob@example.com").unwrap();
        db.create_user("charlie", 42, "charlie@example.com").unwrap();
        db
    }
    
    #[test]
    fn test_list_users() {
        let db = setup_populated_db();
        let users = db.list_users();
        
        assert_eq!(users.len(), 3);
        
        // Check that all users are present (order may vary)
        let names: Vec<&str> = users.iter().map(|u| u.name.as_str()).collect();
        assert!(names.contains(&"alice"));
        assert!(names.contains(&"bob"));
        assert!(names.contains(&"charlie"));
    }
    
    #[test]
    fn test_count() {
        let db = setup_populated_db();
        assert_eq!(db.count(), 3);
    }
}

//------------------------------------------------------
// STRUCT-BASED TEST FIXTURES
//------------------------------------------------------

#[cfg(test)]
mod struct_fixture_tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    
    // A struct-based test fixture for a temporary file
    struct TempFile {
        path: PathBuf,
    }
    
    impl TempFile {
        fn new(name: &str, content: &str) -> Self {
            let path = PathBuf::from(name);
            let mut file = File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            TempFile { path }
        }
        
        fn path(&self) -> &PathBuf {
            &self.path
        }
    }
    
    // Automatically clean up the file when the test is done
    impl Drop for TempFile {
        fn drop(&mut self) {
            fs::remove_file(&self.path).unwrap_or_else(|e| {
                eprintln!("Failed to remove test file: {}", e);
            });
        }
    }
    
    // An example of a test using the struct-based fixture
    #[test]
    fn test_temp_file() {
        let file = TempFile::new("test.txt", "hello world");
        
        // The file should exist
        assert!(file.path().exists());
        
        // Read the content
        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "hello world");
        
        // File is automatically deleted when `file` goes out of scope
    }
}

//------------------------------------------------------
// TEST PARAMETERIZATION
//------------------------------------------------------

#[cfg(test)]
mod parameterized_tests {
    use super::*;
    
    // Helper function to run a parameterized test
    fn test_age_validation(age: u32, expected_valid: bool) {
        let is_valid = if age >= 18 && age <= 120 { true } else { false };
        assert_eq!(is_valid, expected_valid);
    }
    
    #[test]
    fn test_age_validation_cases() {
        // Test cases: (age, expected_valid)
        let test_cases = vec![
            (17, false),  // Too young
            (18, true),   // Minimum age
            (42, true),   // Valid age
            (120, true),  // Maximum age
            (121, false), // Too old
        ];
        
        for (age, expected_valid) in test_cases {
            test_age_validation(age, expected_valid);
        }
    }
    
    #[test]
    fn test_email_validation() {
        // Test cases: (email, valid)
        let test_cases = vec![
            ("alice@example.com", true),
            ("bob@example", true),       // Simple validation just checks for @
            ("invalid-email", false),
            ("charlie@example.org", true),
        ];
        
        for (email, expected_valid) in test_cases {
            let valid = email.contains('@');
            assert_eq!(valid, expected_valid, "Email '{}' validation failed", email);
        }
    }
}

//------------------------------------------------------
// TDD EXAMPLE
//------------------------------------------------------

// TDD process for implementing a user search function:
// 1. First, we write the tests (test-first approach)
// 2. Then, we implement the function to make the tests pass

// Step 1: Write the tests
#[cfg(test)]
mod search_tests {
    use super::*;
    
    // Test fixture
    fn setup_test_db() -> UserDatabase {
        let mut db = UserDatabase::new();
        db.create_user("alice", 28, "alice@example.com").unwrap();
        db.create_user("bob", 35, "bob@example.com").unwrap();
        db.create_user("alicia", 42, "alicia@example.com").unwrap();
        db
    }
    
    #[test]
    fn test_search_exact_match() {
        let db = setup_test_db();
        let results = db.search_users("alice");
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "alice");
    }
    
    #[test]
    fn test_search_partial_match() {
        let db = setup_test_db();
        let results = db.search_users("ali");
        
        assert_eq!(results.len(), 2);
        
        // Results should include alice and alicia
        let names: Vec<&str> = results.iter().map(|u| u.name.as_str()).collect();
        assert!(names.contains(&"alice"));
        assert!(names.contains(&"alicia"));
    }
    
    #[test]
    fn test_search_no_match() {
        let db = setup_test_db();
        let results = db.search_users("david");
        
        assert_eq!(results.len(), 0);
    }
    
    #[test]
    fn test_search_case_insensitive() {
        let db = setup_test_db();
        let results = db.search_users("ALICE");
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "alice");
    }
}

// Step 2: Implement the function to make the tests pass
impl UserDatabase {
    /// Searches for users whose names contain the given query string
    pub fn search_users(&self, query: &str) -> Vec<&User> {
        let query = query.to_lowercase();
        self.users
            .values()
            .filter(|user| user.name.to_lowercase().contains(&query))
            .collect()
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Test Organization
//
// In this exercise, you need to organize the tests for a Calculator struct
// following good test organization principles.
//
// 1. Group tests by operation (add, subtract, multiply, divide)
// 2. Create a test fixture for common setup
// 3. Add appropriate test cases for each operation
// 4. Handle edge cases (like division by zero)

/// A simple calculator that performs basic arithmetic operations.
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }
    
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn divide(&self, a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err("Division by zero".to_string());
        }
        Ok(a / b)
    }
}

// TODO: Organize tests for the Calculator struct here
#[cfg(test)]
mod calculator_tests {
    // Implement your organized tests here
}

// Exercise 2: Implement TDD for a Password Validator
//
// Using TDD, implement a password validator that checks if a password:
// 1. Is at least 8 characters long
// 2. Contains at least one uppercase letter
// 3. Contains at least one lowercase letter
// 4. Contains at least one digit
//
// First, write the tests for each requirement, then implement the validator.

// TODO: Implement tests for the password validator using TDD
#[cfg(test)]
mod password_validator_tests {
    // Implement your tests here
}

// TODO: Implement the password validator function
pub fn validate_password(password: &str) -> bool {
    // Implement the password validator here to make the tests pass
    unimplemented!()
}

// Exercise 3: Struct-Based Test Fixture
//
// Create a struct-based test fixture for a shopping cart 
// that automatically populates the cart with some items
// and ensures proper cleanup.

/// A shopping cart item
#[derive(Debug, Clone, PartialEq)]
pub struct CartItem {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

/// A shopping cart
pub struct ShoppingCart {
    items: Vec<CartItem>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        ShoppingCart { items: Vec::new() }
    }
    
    pub fn add_item(&mut self, name: &str, price: f64, quantity: u32) {
        self.items.push(CartItem {
            name: name.to_string(),
            price,
            quantity,
        });
    }
    
    pub fn get_items(&self) -> &[CartItem] {
        &self.items
    }
    
    pub fn remove_item(&mut self, name: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.name == name) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn total(&self) -> f64 {
        self.items.iter().map(|item| item.price * item.quantity as f64).sum()
    }
}

// TODO: Create a struct-based test fixture for ShoppingCart
#[cfg(test)]
mod shopping_cart_tests {
    use super::*;
    
    // Implement your struct-based test fixture here
    struct TestCart {
        // ...
    }
    
    // Implement tests using the fixture
} 