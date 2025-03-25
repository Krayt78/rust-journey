// Integration Testing in Rust
//
// This file demonstrates the concepts of integration testing in Rust.
// It focuses on how to structure and organize integration tests for a project.

fn main() {
    println!("Integration Testing in Rust");
    println!("This file demonstrates concepts related to integration testing.");
    println!("\nIntegration tests in Rust are typically located in a 'tests' directory");
    println!("at the root of your project, outside the 'src' directory.");
    println!("\nSince integration tests need to be in a specific project structure,");
    println!("the examples in this file will show the concepts and structure rather");
    println!("than executable tests you can run directly from this file.");
    
    println!("\nSee the README.md file in this directory for more information on");
    println!("integration testing in Rust.");
}

//------------------------------------------------------
// PROJECT STRUCTURE FOR INTEGRATION TESTS
//------------------------------------------------------

/*
A typical Rust project with integration tests might have this structure:

my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs       (Library code)
│   └── main.rs      (Optional binary)
└── tests/
    ├── integration_test.rs  (Integration tests)
    └── common/
        └── mod.rs           (Shared test utilities)
*/

//------------------------------------------------------
// EXAMPLE: LIBRARY CODE (what would be in src/lib.rs)
//------------------------------------------------------

// This is an example of what might be in your lib.rs file
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    active: bool, // Note: private field
}

impl User {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    // Private helper method
    fn validate_email(email: &str) -> bool {
        // Simple validation: check if email contains @ and .
        email.contains('@') && email.contains('.')
    }
}

pub mod user_manager {
    use super::User;
    use std::collections::HashMap;
    
    pub struct UserManager {
        users: HashMap<u64, User>,
    }
    
    impl UserManager {
        pub fn new() -> Self {
            UserManager {
                users: HashMap::new(),
            }
        }
        
        pub fn add_user(&mut self, user: User) -> Result<(), String> {
            // Check if user with this ID already exists
            if self.users.contains_key(&user.id) {
                return Err(format!("User with ID {} already exists", user.id));
            }
            
            // Validate email format
            if !super::User::validate_email(&user.email) {
                return Err("Invalid email format".to_string());
            }
            
            self.users.insert(user.id, user);
            Ok(())
        }
        
        pub fn get_user(&self, id: u64) -> Option<&User> {
            self.users.get(&id)
        }
        
        pub fn delete_user(&mut self, id: u64) -> bool {
            self.users.remove(&id).is_some()
        }
        
        pub fn count(&self) -> usize {
            self.users.len()
        }
    }
}

//------------------------------------------------------
// EXAMPLE: INTEGRATION TESTS (what would be in tests/user_tests.rs)
//------------------------------------------------------

/*
// This is an example of what might be in tests/user_tests.rs

// Import the crate (assuming it's named "my_project")
use my_project::{User, user_manager::UserManager};

#[test]
fn test_user_creation() {
    let user = User::new(1, "Alice", "alice@example.com");
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert!(user.is_active());
}

#[test]
fn test_user_deactivation() {
    let mut user = User::new(1, "Alice", "alice@example.com");
    assert!(user.is_active());
    
    user.deactivate();
    assert!(!user.is_active());
}

#[test]
fn test_user_manager_add_user() {
    let mut manager = UserManager::new();
    let user = User::new(1, "Alice", "alice@example.com");
    
    assert_eq!(manager.count(), 0);
    assert!(manager.add_user(user).is_ok());
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_user_manager_add_duplicate_user() {
    let mut manager = UserManager::new();
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(1, "Bob", "bob@example.com"); // Same ID
    
    assert!(manager.add_user(user1).is_ok());
    assert!(manager.add_user(user2).is_err()); // Should fail
}

#[test]
fn test_user_manager_invalid_email() {
    let mut manager = UserManager::new();
    let user = User::new(1, "Alice", "invalid-email"); // Invalid email
    
    assert!(manager.add_user(user).is_err());
}

#[test]
fn test_user_manager_get_user() {
    let mut manager = UserManager::new();
    let user = User::new(1, "Alice", "alice@example.com");
    
    manager.add_user(user).unwrap();
    
    let retrieved_user = manager.get_user(1);
    assert!(retrieved_user.is_some());
    
    let retrieved_user = retrieved_user.unwrap();
    assert_eq!(retrieved_user.name, "Alice");
    assert_eq!(retrieved_user.email, "alice@example.com");
}

#[test]
fn test_user_manager_delete_user() {
    let mut manager = UserManager::new();
    let user = User::new(1, "Alice", "alice@example.com");
    
    manager.add_user(user).unwrap();
    assert_eq!(manager.count(), 1);
    
    assert!(manager.delete_user(1));
    assert_eq!(manager.count(), 0);
    
    // Try to delete non-existent user
    assert!(!manager.delete_user(1));
}
*/

//------------------------------------------------------
// EXAMPLE: SHARED TEST UTILITIES (what would be in tests/common/mod.rs)
//------------------------------------------------------

/*
// This is an example of what might be in tests/common/mod.rs

// A function to create a test user
pub fn create_test_user(id: u64) -> my_project::User {
    my_project::User::new(
        id,
        &format!("User {}", id),
        &format!("user{}@example.com", id)
    )
}

// A function to create a test user manager with some users
pub fn create_populated_manager(user_count: u64) -> my_project::user_manager::UserManager {
    let mut manager = my_project::user_manager::UserManager::new();
    
    for i in 1..=user_count {
        let user = create_test_user(i);
        manager.add_user(user).unwrap();
    }
    
    manager
}

// Mock data for testing
pub struct TestData {
    pub valid_users: Vec<my_project::User>,
    pub invalid_users: Vec<my_project::User>,
}

pub fn get_test_data() -> TestData {
    let valid_users = vec![
        my_project::User::new(1, "Alice", "alice@example.com"),
        my_project::User::new(2, "Bob", "bob@example.com"),
        my_project::User::new(3, "Charlie", "charlie@example.com"),
    ];
    
    let invalid_users = vec![
        my_project::User::new(4, "Invalid1", "invalid-email"),
        my_project::User::new(5, "Invalid2", "another-invalid-email"),
    ];
    
    TestData {
        valid_users,
        invalid_users,
    }
}
*/

//------------------------------------------------------
// EXAMPLE: USING SHARED TEST UTILITIES (what would be in tests/manager_tests.rs)
//------------------------------------------------------

/*
// This is an example of what might be in tests/manager_tests.rs

// Import the crate
use my_project::user_manager::UserManager;

// Import the common module
mod common;

#[test]
fn test_add_multiple_users() {
    let mut manager = UserManager::new();
    let test_data = common::get_test_data();
    
    // Add all valid users
    for user in &test_data.valid_users {
        assert!(manager.add_user(user.clone()).is_ok());
    }
    
    assert_eq!(manager.count(), test_data.valid_users.len());
    
    // Try to add all invalid users
    for user in &test_data.invalid_users {
        assert!(manager.add_user(user.clone()).is_err());
    }
    
    // Count should remain the same
    assert_eq!(manager.count(), test_data.valid_users.len());
}

#[test]
fn test_populated_manager() {
    let manager = common::create_populated_manager(5);
    assert_eq!(manager.count(), 5);
    
    // Check if we can get all users
    for i in 1..=5 {
        let user = manager.get_user(i);
        assert!(user.is_some());
        assert_eq!(user.unwrap().name, format!("User {}", i));
    }
}
*/

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Note: These exercises are designed to help you understand integration testing concepts.
// To actually implement these, you would need to set up a complete Rust project with
// a proper directory structure including a tests directory.

/*
Exercise 1: Basic Integration Test Structure

Describe how you would set up integration tests for a library that provides
mathematical functions like add, subtract, multiply, and divide. What files
would you create, and what tests would you include in each file?

Answer: [Write your answer here]


Exercise 2: Test Organization

For a library that handles user authentication with functions for registration,
login, logout, and password reset, how would you organize your integration tests?
Consider how you might group related tests and what common test utilities you
might need.

Answer: [Write your answer here]


Exercise 3: Integration Testing Strategy

Consider a library for handling a shopping cart with items, quantities, and prices.
The library allows adding items, removing items, calculating totals, and applying
discounts. Describe your strategy for integration testing this library, including
what you would test and how you would organize the tests.

Answer: [Write your answer here]


Exercise 4: Mocking External Services

If your library interacts with external services (like a database or API),
how would you handle this in integration tests? Describe techniques for
mocking or simulating these dependencies.

Answer: [Write your answer here]
*/ 