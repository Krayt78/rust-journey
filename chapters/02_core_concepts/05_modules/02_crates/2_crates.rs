// Crates in Rust
//
// This file demonstrates the concept of crates in Rust.
// A crate is a compilation unit - the smallest amount of code
// that the Rust compiler considers at a time.

fn main() {
    println!("Understanding Rust Crates!");
    
    // This file (2_crates.rs) is the root of a binary crate
    // If this were a real project, it would be src/main.rs
    
    // Binary crates:
    // - Have a main() function
    // - Compile to an executable
    // - Can use library crates
    
    // Let's simulate how we would use functionality from:
    // 1. A library crate in the same package
    // 2. An external crate from crates.io
    
    // Simulating a library crate in the same package
    println!("\n=== Using a Library Crate ===");
    
    // In a real project, this would be defined in src/lib.rs
    mod my_library {
        // Public items that can be used outside the crate
        pub fn public_function() {
            println!("This is a public function in the library crate");
            // We can call private functions within the same crate
            private_function();
        }
        
        // Private items that can only be used within the crate
        fn private_function() {
            println!("This is a private function in the library crate");
        }
        
        pub struct Calculator {
            pub result: i32,
        }
        
        impl Calculator {
            pub fn new() -> Self {
                Calculator { result: 0 }
            }
            
            pub fn add(&mut self, value: i32) {
                self.result += value;
                println!("Added {}. Result is now {}", value, self.result);
            }
        }
    }
    
    // Using the library crate
    my_library::public_function();
    
    let mut calc = my_library::Calculator::new();
    calc.add(5);
    calc.add(10);
    
    // Simulating an external crate
    println!("\n=== Using an External Crate ===");
    
    // In a real project, you would add the crate to Cargo.toml:
    // [dependencies]
    // rand = "0.8.5"
    
    // Then you would use it like this:
    // use rand::Rng;
    // let random_number = rand::thread_rng().gen_range(1..=100);
    
    // For this example, we'll simulate the rand crate
    mod rand {
        pub trait Rng {
            fn gen_range(&mut self, range: std::ops::RangeInclusive<i32>) -> i32;
        }
        
        pub struct ThreadRng {}
        
        impl Rng for ThreadRng {
            fn gen_range(&mut self, range: std::ops::RangeInclusive<i32>) -> i32 {
                // In a real implementation, this would be random
                // Here we'll just return the start of the range
                *range.start()
            }
        }
        
        pub fn thread_rng() -> ThreadRng {
            ThreadRng {}
        }
    }
    
    // Using our simulated external crate
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);
    println!("Random number from simulated crate: {}", random_number);
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Create a Library Crate
// 
// Imagine you're creating a library crate for mathematical operations.
// Complete the code below to provide basic math functions.

mod math_lib {
    // TODO: Implement add, subtract, multiply, and divide functions
    // Make sure they're public so they can be used outside the crate
    
    // Example:
    // pub fn add(a: i32, b: i32) -> i32 {
    //     a + b
    // }
    
    // TODO: Add a function to compute the factorial of a number
    
    // TODO: Add a structure to represent a Point in 2D space
    // with methods to calculate distance from origin and between points
}

// Challenge 2: Using External Crates
//
// Imagine you're building an application that needs to:
// 1. Generate random numbers (using the rand crate)
// 2. Serialize and deserialize data (using the serde crate)
//
// Write code comments below that show:
// 1. What you would add to Cargo.toml
// 2. How you would use these crates in your code

fn challenge_external_crates() {
    // TODO: Add comments showing Cargo.toml entries
    
    // TODO: Add comments showing how to use rand for random numbers
    
    // TODO: Add comments showing how to use serde for serialization
}

// Challenge 3: Conditional Compilation with Features
//
// Imagine you're creating a library with optional features.
// Write code that demonstrates how to:
// 1. Define features in Cargo.toml
// 2. Use conditional compilation based on features

mod feature_example {
    // TODO: Add comments showing how to define features in Cargo.toml
    
    // TODO: Implement functions that are conditionally compiled based on features
    
    // Example:
    // #[cfg(feature = "advanced_math")]
    // pub fn complex_calculation() {
    //     println!("Doing a complex calculation");
    // }
}

// Note: These challenges involve writing code or comments that demonstrate
// understanding of crates. There are no automated tests for them. 