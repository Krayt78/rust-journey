// Paths in Rust
//
// This file demonstrates how to use paths to navigate the module system in Rust.
// Paths allow you to find and reference items (functions, structs, etc.) in the module tree.

fn main() {
    println!("Understanding Rust Paths!");
    
    //------------------------------------------------------
    // ABSOLUTE PATHS
    //------------------------------------------------------
    println!("\n=== Absolute Paths ===");
    
    // Absolute paths start from the crate root
    
    // When using code from the standard library, we use the crate name
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("key", "value");
    println!("Created a HashMap using an absolute path: {:?}", hash_map);
    
    // When referencing items in our own crate, we use the 'crate' keyword
    crate::library::function_in_library();
    
    // Using an absolute path to a module in our crate
    crate::creatures::animals::dog::bark();
    
    //------------------------------------------------------
    // RELATIVE PATHS
    //------------------------------------------------------
    println!("\n=== Relative Paths ===");
    
    // Relative paths start from the current module
    
    // We can reference the library module directly since it's at the same level as main
    library::function_in_library();
    
    // Reference a nested module from the current location
    creatures::animals::cat::meow();
    
    //------------------------------------------------------
    // SELF, SUPER, AND PARENT PATHS
    //------------------------------------------------------
    println!("\n=== Self, Super, and Parent Module Paths ===");
    
    // The 'self' keyword refers to the current module
    // The 'super' keyword refers to the parent module
    // Let's see them in action through the creatures module
    
    creatures::demonstrate_paths();
}

// A simple library module to demonstrate absolute paths
pub mod library {
    pub fn function_in_library() {
        println!("Called a function in the library module");
    }
    
    pub fn demonstrate_absolute_path() {
        // Using an absolute path from within another module
        crate::main();  // This would call the main function if it were public
    }
}

// Module tree to demonstrate different path types
pub mod creatures {
    pub fn demonstrate_paths() {
        println!("Demonstrating different path types:");
        
        // Using self to refer to the current module
        self::animals::dog::bark();
        
        // Using a relative path to a child module
        animals::cat::meow();
        
        // Using super to refer to the parent module (crate root in this case)
        super::library::function_in_library();
    }
    
    pub mod animals {
        pub mod dog {
            pub fn bark() {
                println!("Woof! (from dog module)");
                
                // Using super to access a function in the parent module
                super::animal_noise();
                
                // Using super::super to access a function two levels up
                super::super::identify_creature("dog");
            }
        }
        
        pub mod cat {
            pub fn meow() {
                println!("Meow! (from cat module)");
                
                // Using super to access a function in the parent module
                super::animal_noise();
                
                // Using super::super to access a function two levels up
                super::super::identify_creature("cat");
            }
        }
        
        fn animal_noise() {
            println!("Animals make noises!");
        }
    }
    
    pub mod plants {
        pub fn grow() {
            println!("Growing! (from plants module)");
            
            // Using super to access a function in the parent module
            super::identify_creature("plant");
        }
    }
    
    fn identify_creature(creature_type: &str) {
        println!("Identified a {} in the creatures module", creature_type);
    }
}

//------------------------------------------------------
// DISAMBIGUATION
//------------------------------------------------------

// Let's demonstrate how to handle name conflicts
mod disambiguation_example {
    pub fn run() {
        println!("\n=== Path Disambiguation ===");
        
        // Both modules have a function with the same name
        module_a::shared_name();
        module_b::shared_name();
        
        // When you have items with the same name in scope,
        // you need to specify which one you want to use
        use module_a::shared_name as a_shared_name;
        use module_b::shared_name as b_shared_name;
        
        a_shared_name();
        b_shared_name();
    }
    
    mod module_a {
        pub fn shared_name() {
            println!("Called shared_name from module_a");
        }
    }
    
    mod module_b {
        pub fn shared_name() {
            println!("Called shared_name from module_b");
        }
    }
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Fixing Path References
//
// The following module structure has errors in its path references.
// Fix them by using the appropriate path types (absolute, relative, super, self).

mod challenge_1 {
    pub fn run() -> Result<(), String> {
        println!("\nChallenge 1: Fixing Path References");
        
        mod outer {
            pub fn outer_function() -> &'static str {
                "I'm in the outer module"
            }
            
            pub mod middle {
                pub fn middle_function() -> &'static str {
                    "I'm in the middle module"
                }
                
                pub mod inner {
                    pub fn inner_function() -> &'static str {
                        "I'm in the inner module"
                    }
                    
                    // TODO: Fix this function to call middle_function and outer_function
                    // using the correct paths
                    pub fn call_other_functions() -> (String, String) {
                        // These paths are incorrect - fix them
                        let from_middle = middle_function();
                        let from_outer = outer_function();
                        
                        (from_middle.to_string(), from_outer.to_string())
                    }
                }
            }
        }
        
        // Test the fixed paths
        let (middle, outer) = outer::middle::inner::call_other_functions();
        
        if middle != "I'm in the middle module" || outer != "I'm in the outer module" {
            return Err("Path references are not fixed correctly".to_string());
        }
        
        println!("Challenge 1 completed successfully!");
        Ok(())
    }
}

// Challenge 2: Using Absolute and Relative Paths
//
// Complete the functions in the following module structure to
// demonstrate your understanding of absolute and relative paths.

mod challenge_2 {
    pub fn run() -> Result<(), String> {
        println!("\nChallenge 2: Using Absolute and Relative Paths");
        
        // TODO: Complete these functions to call the target functions
        // using the specified path type
        fn call_with_absolute_path() -> String {
            // Use an absolute path to call target_function
            "".to_string()
        }
        
        fn call_with_relative_path() -> String {
            // Use a relative path to call target_function
            "".to_string()
        }
        
        mod nested {
            pub fn target_function() -> &'static str {
                "You found the target function!"
            }
            
            // TODO: Complete this function to call target_function using self
            pub fn call_with_self() -> String {
                "".to_string()
            }
        }
        
        // Test the paths
        let absolute = call_with_absolute_path();
        let relative = call_with_relative_path();
        let self_path = nested::call_with_self();
        
        let expected = "You found the target function!";
        
        if absolute != expected || relative != expected || self_path != expected {
            return Err("Path references are not implemented correctly".to_string());
        }
        
        println!("Challenge 2 completed successfully!");
        Ok(())
    }
}

// Challenge 3: Path Disambiguation
//
// Fix the code below to handle name conflicts by using
// appropriate disambiguation techniques.

mod challenge_3 {
    pub fn run() -> Result<(), String> {
        println!("\nChallenge 3: Path Disambiguation");
        
        mod geometry {
            pub struct Point {
                pub x: f64,
                pub y: f64,
            }
            
            impl Point {
                pub fn new(x: f64, y: f64) -> Self {
                    Point { x, y }
                }
            }
        }
        
        mod graphics {
            pub struct Point {
                pub x: i32,
                pub y: i32,
                pub color: String,
            }
            
            impl Point {
                pub fn new(x: i32, y: i32, color: &str) -> Self {
                    Point { x, y, color: color.to_string() }
                }
            }
        }
        
        // TODO: Fix this function to use both Point types
        // Hint: Use the 'use' statement with 'as' to rename
        fn use_both_points() -> (String, String) {
            // Create a geometry Point
            let geo_point = Point::new(1.0, 2.0);
            
            // Create a graphics Point
            let gfx_point = Point::new(1, 2, "red");
            
            // Return string representations of both points
            (
                format!("Geometry point: ({}, {})", geo_point.x, geo_point.y),
                format!("Graphics point: ({}, {}, {})", gfx_point.x, gfx_point.y, gfx_point.color)
            )
        }
        
        // Test the disambiguation
        let (geo, gfx) = use_both_points();
        
        if geo != "Geometry point: (1, 2)" || gfx != "Graphics point: (1, 2, red)" {
            return Err("Name conflicts are not resolved correctly".to_string());
        }
        
        println!("Challenge 3 completed successfully!");
        Ok(())
    }
}

// A function to run all challenges
pub fn run_challenges() {
    println!("\nRunning path challenges...");
    
    if let Err(e) = challenge_1::run() {
        println!("Challenge 1 failed: {}", e);
    }
    
    if let Err(e) = challenge_2::run() {
        println!("Challenge 2 failed: {}", e);
    }
    
    if let Err(e) = challenge_3::run() {
        println!("Challenge 3 failed: {}", e);
    }
    
    println!("Path challenges complete!");
    
    // Also run the disambiguation example
    disambiguation_example::run();
} 