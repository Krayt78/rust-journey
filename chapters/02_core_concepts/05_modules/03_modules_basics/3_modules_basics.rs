// Module Basics in Rust
//
// This file demonstrates the basic concepts of modules in Rust.
// Modules help you organize code within a crate for readability and reusability.

fn main() {
    println!("Understanding Rust Modules!");
    
    //------------------------------------------------------
    // DEFINING MODULES
    //------------------------------------------------------
    println!("\n=== Defining Modules ===");
    
    // Modules are defined using the 'mod' keyword
    mod garden {
        // Public items can be accessed from outside the module
        pub fn plant() {
            println!("Planting in the garden!");
        }
        
        // Private items (default) can only be accessed within the module
        fn water() {
            println!("Watering the garden!");
        }
        
        // Public function that calls a private function
        pub fn tend() {
            println!("Tending the garden...");
            water(); // Can access private functions within the same module
        }
    }
    
    // Using the module
    garden::plant();
    garden::tend();
    
    // This would cause an error - can't access private items
    // garden::water();
    
    //------------------------------------------------------
    // NESTED MODULES
    //------------------------------------------------------
    println!("\n=== Nested Modules ===");
    
    mod farm {
        pub fn overview() {
            println!("The farm has animals and crops!");
        }
        
        // A nested module for animals
        pub mod animals {
            pub fn list() {
                println!("Farm animals: cows, chickens, sheep");
            }
            
            pub fn care() {
                println!("Taking care of farm animals...");
                feed();
                // We can call functions from the parent module using 'super'
                super::show_farm_status();
            }
            
            fn feed() {
                println!("Feeding the animals");
            }
        }
        
        // A nested module for crops
        pub mod crops {
            pub fn list() {
                println!("Farm crops: corn, wheat, soybeans");
            }
            
            pub fn plant() {
                println!("Planting crops...");
                // We can call functions from the parent module using 'super'
                super::show_farm_status();
            }
        }
        
        // Private function that can be called by nested modules
        fn show_farm_status() {
            println!("Farm status: all good!");
        }
    }
    
    // Using nested modules
    farm::overview();
    farm::animals::list();
    farm::animals::care();
    farm::crops::list();
    farm::crops::plant();
    
    //------------------------------------------------------
    // MODULE ORGANIZATION
    //------------------------------------------------------
    println!("\n=== Module Organization ===");
    
    // In a real project, modules would often be in separate files.
    // Here's how that might look if everything was in one file:
    
    // File: src/lib.rs or src/main.rs
    mod school {
        pub mod students {
            pub fn enroll() {
                println!("Enrolling a student");
            }
        }
        
        pub mod courses {
            pub fn add() {
                println!("Adding a course");
            }
        }
    }
    
    // In separate files, this would be:
    // 
    // File: src/lib.rs or src/main.rs
    // mod school;
    //
    // File: src/school.rs
    // pub mod students;
    // pub mod courses;
    //
    // File: src/school/students.rs
    // pub fn enroll() {
    //     println!("Enrolling a student");
    // }
    //
    // File: src/school/courses.rs
    // pub fn add() {
    //     println!("Adding a course");
    // }
    
    // Using the modules
    school::students::enroll();
    school::courses::add();
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Create Module Hierarchy
//
// Complete this module hierarchy for a bookstore:
// - bookstore (main module)
//   - books (submodule with functions to add, list, find books)
//   - customers (submodule with functions to add, list, find customers)
//   - sales (submodule with functions to make a sale, list sales)

mod bookstore {
    // TODO: Implement the bookstore module and its submodules
    // Make appropriate functions public
}

// Challenge 2: Module Organization
//
// Imagine you're creating a game with these components:
// - Player
// - Enemies
// - Items
// - World
//
// Create a module structure to organize the game components.
// Think about what should be public vs private.

mod game {
    // TODO: Implement a sensible module structure for the game
}

// Challenge 3: Using super and self
//
// Complete this module to demonstrate the use of 'super' and 'self'
// to reference items in the module hierarchy.

mod university {
    pub fn get_name() -> &'static str {
        "Rust University"
    }
    
    pub mod departments {
        // TODO: Implement a function that uses super to access get_name
        
        pub mod computer_science {
            // TODO: Implement a function that uses super::super to access get_name
        }
    }
}

// Challenge 4: Re-exporting
//
// Demonstrate how to re-export items from a nested module
// to make the API more accessible.

mod api {
    // Implementation details hidden in nested modules
    mod internals {
        pub fn helper_function() {
            println!("This is a helper function!");
        }
        
        pub struct Config {
            pub debug_mode: bool,
        }
    }
    
    // TODO: Re-export the helper_function and Config 
    // so they can be used as api::helper_function and api::Config
}

// You can test your implementation by uncommenting and running these lines:
/*
fn test_challenges() {
    // Challenge 1
    bookstore::books::add();
    bookstore::customers::add();
    bookstore::sales::make_sale();
    
    // Challenge 2
    game::player::move_player();
    game::enemies::spawn();
    
    // Challenge 3
    println!("{}", university::departments::get_university_name());
    println!("{}", university::departments::computer_science::get_university_name());
    
    // Challenge 4
    api::helper_function();
    let config = api::Config { debug_mode: true };
}
*/ 