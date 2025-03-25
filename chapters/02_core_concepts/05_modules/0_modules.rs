// Managing Growing Projects with Packages, Crates, and Modules
//
// This program demonstrates Rust's module system, including:
// - Defining and organizing modules
// - Setting visibility with pub
// - Creating and using paths
// - Using the 'use' keyword
// - Structuring modules with files
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// In the main file, we'll focus on examples that work within a single file.
// In a real project, you'd typically split this across multiple files.

fn main() {
    println!("Exploring Modules in Rust!");
    
    //------------------------------------------------------
    // BASIC MODULE DEFINITIONS
    //------------------------------------------------------
    println!("\n--- Basic Module Definitions ---");
    
    // Modules are defined with the 'mod' keyword
    mod garden {
        // By default, everything in a module is private
        pub fn plant_seed() {
            println!("Planting a seed in the garden!");
        }
        
        // This function is private to the module
        fn water_plant() {
            println!("Watering a plant in the garden!");
        }
        
        // Submodules can access private items in their parent modules
        pub mod vegetables {
            pub fn harvest() {
                println!("Harvesting vegetables!");
                // We can call the private function from the parent module
                super::water_plant();
            }
        }
    }
    
    // Call a public function in a module
    garden::plant_seed();
    
    // Call a function in a submodule
    garden::vegetables::harvest();
    
    //------------------------------------------------------
    // ABSOLUTE AND RELATIVE PATHS
    //------------------------------------------------------
    println!("\n--- Absolute and Relative Paths ---");
    
    mod house {
        pub mod bedroom {
            pub fn make_bed() {
                println!("Making the bed!");
                // Here are three equivalent ways to call the same function:
                
                // 1. Absolute path from crate root
                crate::house::kitchen::wash_dishes();
                
                // 2. Relative path using super to go up one level
                super::kitchen::wash_dishes();
                
                // 3. Relative path using a binding from use
                use super::kitchen::wash_dishes;
                wash_dishes();
            }
        }
        
        pub mod kitchen {
            pub fn wash_dishes() {
                println!("Washing the dishes!");
            }
            
            pub fn cook() {
                println!("Cooking dinner!");
                // Use super to access parent module
                super::dining::set_table();
            }
        }
        
        pub mod dining {
            pub fn set_table() {
                println!("Setting the table!");
            }
            
            pub fn eat() {
                println!("Eating dinner!");
                // Use an absolute path
                crate::house::kitchen::wash_dishes();
            }
        }
    }
    
    house::bedroom::make_bed();
    house::kitchen::cook();
    house::dining::eat();
    
    //------------------------------------------------------
    // BRINGING PATHS INTO SCOPE WITH 'USE'
    //------------------------------------------------------
    println!("\n--- Using the 'use' Keyword ---");
    
    // The 'use' keyword brings paths into scope
    use crate::house::kitchen;
    
    // Now we can use the shorter path
    kitchen::cook();
    
    // For functions, it's idiomatic to bring the parent module into scope
    use crate::house::dining;
    dining::eat();
    
    // For structs and enums, it's idiomatic to use the full path
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("Scores: {:?}", scores);
    
    //------------------------------------------------------
    // NESTED PATHS
    //------------------------------------------------------
    println!("\n--- Nested Paths ---");
    
    // Import multiple items from the same module
    use std::io::{self, Write};
    
    // This is equivalent to:
    // use std::io;
    // use std::io::Write;
    
    //------------------------------------------------------
    // PUBLIC STRUCTS AND ENUMS
    //------------------------------------------------------
    println!("\n--- Public Structs and Enums ---");
    
    mod plant_life {
        // Public struct with some public and some private fields
        pub struct Plant {
            pub name: String,      // Public field
            pub species: String,   // Public field
            id: u32,               // Private field
        }
        
        impl Plant {
            pub fn new(name: String, species: String) -> Plant {
                Plant {
                    name,
                    species,
                    id: 1,  // Private field initialized in constructor
                }
            }
            
            pub fn description(&self) -> String {
                format!("{} ({}), ID: {}", self.name, self.species, self.id)
            }
        }
        
        // Public enum with public variants
        pub enum PlantType {
            Flowering,
            NonFlowering,
            Fern,
            Moss,
        }
    }
    
    // Create an instance of a public struct
    use plant_life::Plant;
    let plant = Plant::new(
        String::from("Venus Flytrap"),
        String::from("Dionaea muscipula"),
    );
    
    println!("Plant: {}", plant.description());
    
    // Access public fields
    println!("Plant name: {}", plant.name);
    
    // This would cause an error - can't access a private field:
    // println!("Plant ID: {}", plant.id);
    
    //------------------------------------------------------
    // RE-EXPORTING WITH PUB USE
    //------------------------------------------------------
    println!("\n--- Re-exporting with pub use ---");
    
    mod biology {
        pub mod plants {
            pub fn study() {
                println!("Studying plants (botany)");
            }
        }
        
        pub mod animals {
            pub fn study() {
                println!("Studying animals (zoology)");
            }
        }
        
        // Re-export plants::study as a direct function in biology
        pub use self::plants::study as study_plants;
        
        // Re-export at the top level
        pub use self::animals::study as study_animals;
    }
    
    // We can call the re-exported function directly
    biology::study_plants();
    biology::study_animals();
    
    // We can also still use the original paths
    biology::plants::study();
    biology::animals::study();
}

//------------------------------------------------------
// Library modules that would normally be in separate files
//------------------------------------------------------

// In a real project, these would be in separate files.
// We're including them here for demonstration purposes.

mod restaurant {
    // A module that models a restaurant
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Adding customer to waitlist");
            }
            
            pub fn seat_at_table() {
                println!("Seating customer at table");
            }
        }
        
        pub mod serving {
            pub fn take_order() {
                println!("Taking customer order");
            }
            
            pub fn serve_order() {
                println!("Serving customer order");
            }
            
            pub fn take_payment() {
                println!("Taking payment from customer");
            }
        }
    }
    
    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
        
        pub enum Appetizer {
            Soup,
            Salad,
        }
        
        pub fn fix_incorrect_order() {
            cook_order();
            super::front_of_house::serving::serve_order();
        }
        
        fn cook_order() {
            println!("Cooking customer order");
        }
    }
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::restaurant::front_of_house::hosting::add_to_waitlist();
        
        // Relative path
        front_of_house::hosting::seat_at_table();
        
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        
        // Change our mind about the type of bread
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        
        // This would cause an error:
        // meal.seasonal_fruit = String::from("blueberries");
        
        // Order an appetizer
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
        
        // Fix an incorrect order
        back_of_house::fix_incorrect_order();
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // Challenge 1: Fix the visibility issues in this module hierarchy
    pub fn challenge_visibility() -> Result<(), String> {
        mod car_factory {
            mod engine {
                fn build_engine() -> String {
                    String::from("Engine built")
                }
            }
            
            mod body {
                fn build_body() -> String {
                    String::from("Body built")
                }
            }
            
            mod assembly {
                fn assemble_car() -> String {
                    // These paths are incorrect - fix them
                    let engine = engine::build_engine();
                    let body = body::build_body();
                    
                    format!("{}, {}, Car assembled!", engine, body)
                }
            }
            
            // This function should call assemble_car and return the result
            fn manufacture() -> String {
                assembly::assemble_car()
            }
        }
        
        // This call should work after you fix the visibility issues
        let result = car_factory::manufacture();
        
        if result != "Engine built, Body built, Car assembled!" {
            return Err(format!("Expected 'Engine built, Body built, Car assembled!', got '{}'", result));
        }
        
        Ok(())
    }
    
    // Challenge 2: Fix the use statements and paths
    pub fn challenge_use_paths() -> Result<(), String> {
        mod garden {
            pub mod vegetables {
                pub struct Vegetable {
                    pub name: String,
                    pub price: f64,
                }
                
                impl Vegetable {
                    pub fn new(name: &str, price: f64) -> Vegetable {
                        Vegetable {
                            name: String::from(name),
                            price,
                        }
                    }
                }
            }
            
            pub mod fruits {
                pub struct Fruit {
                    pub name: String,
                    pub price: f64,
                }
                
                impl Fruit {
                    pub fn new(name: &str, price: f64) -> Fruit {
                        Fruit {
                            name: String::from(name),
                            price,
                        }
                    }
                }
            }
        }
        
        // Fix these use statements to follow Rust's idiomatic style
        use garden::vegetables;
        use garden::fruits;
        
        // Fix these to use the correct paths after modifying the use statements
        let carrot = Vegetable::new("Carrot", 1.50);
        let apple = Fruit::new("Apple", 0.75);
        
        let total = carrot.price + apple.price;
        
        if total != 2.25 {
            return Err(format!("Expected total to be 2.25, got {}", total));
        }
        
        Ok(())
    }
    
    // Challenge 3: Fix the pub use statements
    pub fn challenge_pub_use() -> Result<(), String> {
        mod math_operations {
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            fn subtract(a: i32, b: i32) -> i32 {
                a - b
            }
            
            fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
            
            fn divide(a: i32, b: i32) -> Option<i32> {
                if b == 0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
        
        // Fix this module to re-export the functions from math_operations
        // using pub use statements
        mod calculator {
            // Import and re-export add and subtract from math_operations
            
            // Define a new function that uses multiply
            pub fn calculate_area(width: i32, height: i32) -> i32 {
                multiply(width, height)
            }
            
            // Define a new function that uses divide
            pub fn calculate_ratio(a: i32, b: i32) -> Option<i32> {
                divide(a, b)
            }
        }
        
        // These should work after fixing the pub use statements
        let sum = calculator::add(5, 10);
        let difference = calculator::subtract(10, 5);
        let area = calculator::calculate_area(4, 5);
        let ratio = calculator::calculate_ratio(10, 2);
        
        if sum != 15 || difference != 5 || area != 20 || ratio != Some(5) {
            return Err(format!(
                "Incorrect calculations. sum: {}, difference: {}, area: {}, ratio: {:?}",
                sum, difference, area, ratio
            ));
        }
        
        Ok(())
    }
    
    // Challenge 4: Fix nested module access
    pub fn challenge_nested_modules() -> Result<(), String> {
        mod authentication {
            mod credentials {
                pub struct User {
                    username: String,
                    password: String,
                }
                
                impl User {
                    pub fn new(username: &str, password: &str) -> User {
                        User {
                            username: String::from(username),
                            password: String::from(password),
                        }
                    }
                    
                    pub fn get_username(&self) -> &str {
                        &self.username
                    }
                    
                    fn verify_password(&self, password: &str) -> bool {
                        self.password == password
                    }
                }
            }
            
            mod validation {
                fn is_valid_username(username: &str) -> bool {
                    username.len() >= 3
                }
                
                pub fn validate_credentials(user: User, password: &str) -> bool {
                    if is_valid_username(user.get_username()) {
                        user.verify_password(password)
                    } else {
                        false
                    }
                }
            }
            
            pub fn login(username: &str, password: &str) -> bool {
                let user = credentials::User::new(username, password);
                validation::validate_credentials(user, password)
            }
        }
        
        // Fix the issues to make this work
        let logged_in = authentication::login("admin", "password123");
        
        if !logged_in {
            return Err("Login should succeed".to_string());
        }
        
        Ok(())
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Module challenges...");
        
        // Challenge 1: Fix visibility issues
        if let Err(e) = challenges::challenge_visibility() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the visibility issues!");
        
        // Challenge 2: Fix use paths
        if let Err(e) = challenges::challenge_use_paths() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the use path issues!");
        
        // Challenge 3: Fix pub use
        if let Err(e) = challenges::challenge_pub_use() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the pub use issues!");
        
        // Challenge 4: Fix nested module access
        if let Err(e) = challenges::challenge_nested_modules() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the nested module access issues!");
        
        println!("All Module challenges completed successfully!");
        Ok(())
    }
} 