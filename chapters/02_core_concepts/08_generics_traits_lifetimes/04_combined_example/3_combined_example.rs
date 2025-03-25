// Combined Example: Generics, Traits, and Lifetimes
//
// This file demonstrates how to combine generics, traits, and lifetimes
// in Rust to create flexible, reusable, and safe code.

use std::fmt::{Display, Debug};

fn main() {
    println!("Combining Generics, Traits, and Lifetimes in Rust!");
    
    //------------------------------------------------------
    // BASIC EXAMPLE: A GENERIC DATA CONTAINER
    //------------------------------------------------------
    println!("\n=== Basic Example: A Generic Data Container ===");
    
    // Creating instances of our generic container with different types
    let string_container = Container::new(String::from("Hello, Rust!"));
    let int_container = Container::new(42);
    let float_container = Container::new(3.14);
    
    // Printing the containers (using derived Debug trait)
    println!("String container: {:?}", string_container);
    println!("Integer container: {:?}", int_container);
    println!("Float container: {:?}", float_container);
    
    // Using type-specific methods
    println!("String length: {}", string_container.get_string_length());
    println!("Integer is positive: {}", int_container.is_positive());
    println!("Float is integer: {}", float_container.is_integer());
    
    //------------------------------------------------------
    // ADVANCED EXAMPLE: DATA PROCESSOR WITH REFERENCES
    //------------------------------------------------------
    println!("\n=== Advanced Example: Data Processor with References ===");
    
    // Source data
    let data = vec![1, 2, 3, 4, 5];
    let description = String::from("Sample data for processing");
    
    // Create a data processor that references our data
    let processor = DataProcessor {
        data: &data,
        description: &description,
    };
    
    // Process the data in different ways
    println!("Sum: {}", processor.process(|data| {
        data.iter().sum::<i32>()
    }));
    
    println!("Product: {}", processor.process(|data| {
        data.iter().product::<i32>()
    }));
    
    println!("Max: {}", processor.process(|data| {
        *data.iter().max().unwrap_or(&0)
    }));
    
    // Print the processor's description
    println!("Description: {}", processor.get_description());
    
    //------------------------------------------------------
    // PRACTICAL EXAMPLE: GENERIC REPOSITORY PATTERN
    //------------------------------------------------------
    println!("\n=== Practical Example: Generic Repository Pattern ===");
    
    // Create a user and a product
    let user = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    
    let product = Product {
        id: 101,
        name: String::from("Rust Book"),
        price: 29.99,
    };
    
    // Create repositories for users and products
    let user_repo = InMemoryRepository::<User>::new();
    let mut product_repo = InMemoryRepository::<Product>::new();
    
    // Save items to repositories
    let user_id = 1;
    let product_id = 101;
    
    user_repo.save(user_id, user);
    product_repo.save(product_id, product);
    
    // Retrieve and display items
    match user_repo.find_by_id(user_id) {
        Some(user) => println!("Found user: {}", user),
        None => println!("User not found"),
    }
    
    match product_repo.find_by_id(product_id) {
        Some(product) => println!("Found product: {}", product),
        None => println!("Product not found"),
    }
    
    // Update a product
    if let Some(mut product) = product_repo.find_by_id(product_id) {
        product.price = 24.99;
        product_repo.save(product_id, product);
        
        if let Some(updated_product) = product_repo.find_by_id(product_id) {
            println!("Updated product price: ${:.2}", updated_product.price);
        }
    }
    
    //------------------------------------------------------
    // FINAL EXAMPLE: COMBINED API CLIENT
    //------------------------------------------------------
    println!("\n=== Final Example: API Client with Generic Response Handling ===");
    
    // Create a configuration for our API client
    let config = ApiConfig {
        base_url: String::from("https://api.example.com"),
        timeout: 30,
        retry_count: 3,
    };
    
    // Create an API client with a reference to the configuration
    let client = ApiClient::new(&config);
    
    // Simulate fetching a user
    if let Ok(fetch_result) = client.fetch::<User>("/users/1") {
        println!("Fetched user: {}", fetch_result.data);
        println!("Response metadata: {:?}", fetch_result.metadata);
    }
    
    // Simulate fetching a product
    if let Ok(fetch_result) = client.fetch::<Product>("/products/101") {
        println!("Fetched product: {}", fetch_result.data);
        println!("Response metadata: {:?}", fetch_result.metadata);
    }
    
    // Example of error handling with generic types
    match client.fetch::<User>("/non-existent") {
        Ok(result) => println!("Successfully fetched: {}", result.data),
        Err(error) => println!("Error: {}", error),
    }
}

//------------------------------------------------------
// BASIC EXAMPLE IMPLEMENTATIONS
//------------------------------------------------------

// A generic container that can hold any type
#[derive(Debug)]
struct Container<T> {
    value: T,
}

// Generic implementation for any type
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

// Specialized implementation for String types
impl Container<String> {
    fn get_string_length(&self) -> usize {
        self.value.len()
    }
}

// Specialized implementation for integer types (i32)
impl Container<i32> {
    fn is_positive(&self) -> bool {
        self.value > 0
    }
}

// Specialized implementation for float types (f64)
impl Container<f64> {
    fn is_integer(&self) -> bool {
        self.value.fract() == 0.0
    }
}

//------------------------------------------------------
// ADVANCED EXAMPLE IMPLEMENTATIONS
//------------------------------------------------------

// A struct that holds references to data (requires lifetime annotations)
struct DataProcessor<'a, T> {
    data: &'a [T],
    description: &'a str,
}

// Implementation with lifetime parameters
impl<'a, T> DataProcessor<'a, T> {
    // Process the data using a function that takes a slice of T and returns R
    fn process<F, R>(&self, processor_fn: F) -> R
    where
        F: Fn(&[T]) -> R,
    {
        processor_fn(self.data)
    }
    
    // Return the description
    fn get_description(&self) -> &str {
        self.description
    }
}

//------------------------------------------------------
// PRACTICAL EXAMPLE IMPLEMENTATIONS
//------------------------------------------------------

// A trait that defines common methods for entities
trait Entity: Display + Debug {
    fn get_id(&self) -> usize;
}

// User entity
#[derive(Debug, Clone)]
struct User {
    id: usize,
    name: String,
    email: String,
}

// Implement Entity trait for User
impl Entity for User {
    fn get_id(&self) -> usize {
        self.id
    }
}

// Implement Display for User
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ id: {}, name: {}, email: {} }}", self.id, self.name, self.email)
    }
}

// Product entity
#[derive(Debug, Clone)]
struct Product {
    id: usize,
    name: String,
    price: f64,
}

// Implement Entity trait for Product
impl Entity for Product {
    fn get_id(&self) -> usize {
        self.id
    }
}

// Implement Display for Product
impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Product {{ id: {}, name: {}, price: ${:.2} }}", self.id, self.name, self.price)
    }
}

// A trait that defines repository operations
trait Repository<T: Entity> {
    fn find_by_id(&self, id: usize) -> Option<T>;
    fn save(&mut self, id: usize, entity: T);
    fn delete(&mut self, id: usize) -> bool;
}

// In-memory implementation of the Repository trait
struct InMemoryRepository<T: Entity> {
    items: std::collections::HashMap<usize, T>,
}

// Implement methods for InMemoryRepository
impl<T: Entity + Clone> InMemoryRepository<T> {
    fn new() -> Self {
        InMemoryRepository {
            items: std::collections::HashMap::new(),
        }
    }
}

// Implement the Repository trait for InMemoryRepository
impl<T: Entity + Clone> Repository<T> for InMemoryRepository<T> {
    fn find_by_id(&self, id: usize) -> Option<T> {
        self.items.get(&id).cloned()
    }
    
    fn save(&mut self, id: usize, entity: T) {
        self.items.insert(id, entity);
    }
    
    fn delete(&mut self, id: usize) -> bool {
        self.items.remove(&id).is_some()
    }
}

//------------------------------------------------------
// FINAL EXAMPLE IMPLEMENTATIONS
//------------------------------------------------------

// API configuration
struct ApiConfig {
    base_url: String,
    timeout: u32,
    retry_count: u32,
}

// API client that references a configuration
struct ApiClient<'a> {
    config: &'a ApiConfig,
}

// Response metadata
#[derive(Debug)]
struct ResponseMetadata {
    status_code: u16,
    response_time_ms: u64,
    cache_hit: bool,
}

// Generic API response that contains data of type T
struct ApiResponse<T> {
    data: T,
    metadata: ResponseMetadata,
}

// API error type
struct ApiError(String);

// Implement Display for ApiError
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.0)
    }
}

// Implementation of ApiClient
impl<'a> ApiClient<'a> {
    fn new(config: &'a ApiConfig) -> Self {
        ApiClient { config }
    }
    
    // Fetch data of type T from the API
    fn fetch<T: Entity + Clone>(&self, endpoint: &str) -> Result<ApiResponse<T>, ApiError> {
        // In a real implementation, this would make an HTTP request
        // For this example, we'll simulate responses based on the endpoint
        
        println!("Fetching from: {}{}", self.config.base_url, endpoint);
        println!("Timeout: {} seconds", self.config.timeout);
        println!("Retries: {}", self.config.retry_count);
        
        // Simulate different responses based on the endpoint
        if endpoint == "/users/1" {
            // Simulate a successful user response
            let user = User {
                id: 1,
                name: String::from("John Doe"),
                email: String::from("john@example.com"),
            };
            
            let metadata = ResponseMetadata {
                status_code: 200,
                response_time_ms: 120,
                cache_hit: false,
            };
            
            // This is a bit of a hack for the example
            // In a real implementation we'd have proper type checking
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<User>() {
                return Ok(ApiResponse {
                    data: unsafe { std::mem::transmute_copy(&user) },
                    metadata,
                });
            }
        } else if endpoint == "/products/101" {
            // Simulate a successful product response
            let product = Product {
                id: 101,
                name: String::from("Rust Programming"),
                price: 39.99,
            };
            
            let metadata = ResponseMetadata {
                status_code: 200,
                response_time_ms: 85,
                cache_hit: true,
            };
            
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Product>() {
                return Ok(ApiResponse {
                    data: unsafe { std::mem::transmute_copy(&product) },
                    metadata,
                });
            }
        }
        
        // Simulate an error for any other endpoint
        Err(ApiError(format!("Resource not found at {}", endpoint)))
    }
}

//------------------------------------------------------
// CHALLENGE: IMPLEMENT YOUR OWN COMBINED EXAMPLE
//------------------------------------------------------

// Challenge: Build a Logger System
//
// Create a flexible logging system that uses generics, traits, and lifetimes.
// Your implementation should:
// 
// 1. Define a Logger trait with methods for logging different levels (info, warn, error)
// 2. Create different logger implementations (ConsoleLogger, FileLogger)
// 3. Make loggers generic over a Formatter<T> where T is the message type
// 4. Allow references to external configuration with appropriate lifetimes
// 5. Define a LoggableEvent trait that different event types can implement
//
// Bonus: Add a feature to buffer logs and flush them all at once

// TODO: Implement your logging system here 