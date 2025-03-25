// Packages in Rust
//
// This file demonstrates the concept of packages in Rust.
// In a real project, you would typically have multiple files
// organized according to the package structure.

fn main() {
    println!("Understanding Rust Packages!");
    
    // In a typical Rust package, the structure would be:
    //
    // my_package/
    // ├── Cargo.toml       // Package manifest
    // ├── src/
    //     ├── main.rs      // Binary crate root (this file)
    //     ├── lib.rs       // Optional library crate root
    //     └── bin/         // Additional binaries
    //         └── tool.rs  // Another binary
    
    // This main.rs file is the entry point for the default binary crate
    println!("This is the main binary entry point.");
    
    // To create a new package:
    // $ cargo new my_package
    // 
    // To create a library package:
    // $ cargo new my_library --lib
    
    // Let's simulate what a package structure might look like:
    demonstrate_package_structure();
}

// This function demonstrates how a package might be structured
fn demonstrate_package_structure() {
    println!("\n=== Package Structure Example ===");
    
    // In a real package, we would have modules that correspond to files
    
    // This would typically be in lib.rs or a module file
    mod library_code {
        pub fn useful_function() {
            println!("This would be part of the library crate.");
        }
        
        pub struct UsefulStruct {
            pub field: i32,
        }
        
        impl UsefulStruct {
            pub fn new(value: i32) -> Self {
                UsefulStruct { field: value }
            }
        }
    }
    
    // This would typically be a separate binary in the bin/ directory
    mod another_binary {
        use super::library_code;
        
        pub fn run() {
            println!("This would be another binary in the package.");
            
            // Using functionality from the library part of the package
            library_code::useful_function();
            
            let instance = library_code::UsefulStruct::new(42);
            println!("Created a struct with field value: {}", instance.field);
        }
    }
    
    // Simulate running the second binary
    another_binary::run();
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Note: These challenges are designed to be conceptual and don't involve actual coding.
// The goal is to understand how to design and structure Rust packages.
// There are no "unit tests" for these challenges since they're about design concepts.
// Discuss your solutions with others or check a reference implementation for comparison. 

// Challenge 1: Design a Package Structure
//
// Imagine you're creating a command-line calculator application.
// The application should have:
// - A library crate with core functionality
// - A main binary that uses the library
// - A second binary for a "scientific mode"
//
// Describe how you would structure this package by filling in the comments below.

mod challenge_1 {
    pub fn design_calculator_package() {
        // TODO: Replace these comments with a description of the package structure
        
        // 1. What would your Cargo.toml contain?
        
        // 2. What files would you create in the src/ directory?
        
        // 3. What modules would you define in each file?
        
        // 4. How would the binaries use the library code?
    }
}

// Challenge 2: Workspace Structure
//
// Imagine you're building a larger application with multiple related components.
// Design a workspace structure for a web application that includes:
// - A core library with shared functionality
// - A web server package
// - A CLI tool package for administration
// - A package for database migrations

mod challenge_2 {
    pub fn design_workspace_structure() {
        // TODO: Replace these comments with a description of the workspace structure
        
        // 1. What would your root Cargo.toml contain?
        
        // 2. What packages would you create?
        
        // 3. How would the packages depend on each other?
        
        // 4. How would you share code between packages?
    }
}