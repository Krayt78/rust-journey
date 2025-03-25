// Privacy and Visibility in Rust
//
// This file demonstrates how privacy and visibility rules work in Rust's module system.
// Privacy controls what code can access which items in the module hierarchy.

fn main() {
    println!("Understanding Privacy and Visibility in Rust!");
    
    //------------------------------------------------------
    // BASIC PRIVACY RULES
    //------------------------------------------------------
    println!("\n=== Basic Privacy Rules ===");
    
    // By default, everything in Rust is private
    mod garden {
        // Public function - accessible from outside the module
        pub fn plant_seeds() {
            println!("Planting seeds in the garden!");
            
            // We can call private functions within the same module
            water_garden();
        }
        
        // Private function - only accessible within this module and child modules
        fn water_garden() {
            println!("Watering the garden!");
        }
        
        // Child module can access parent's private items
        pub mod vegetables {
            pub fn plant_vegetables() {
                println!("Planting vegetables!");
                
                // We can access the parent module's private function
                super::water_garden();
            }
        }
    }
    
    // Using the public function
    garden::plant_seeds();
    
    // Using the public function in a child module
    garden::vegetables::plant_vegetables();
    
    // This would cause an error - can't access private function
    // garden::water_garden();
    
    //------------------------------------------------------
    // STRUCT PRIVACY
    //------------------------------------------------------
    println!("\n=== Struct Privacy ===");
    
    mod plants {
        // Public struct with mixed field privacy
        pub struct Plant {
            pub name: String,        // Public field
            pub species: String,      // Public field
            water_frequency: u32,     // Private field
        }
        
        impl Plant {
            // Public constructor
            pub fn new(name: &str, species: &str, water_frequency: u32) -> Plant {
                Plant {
                    name: name.to_string(),
                    species: species.to_string(),
                    water_frequency,  // We can set private fields within the module
                }
            }
            
            // Public method
            pub fn description(&self) -> String {
                format!("{} ({}), Water every {} days", 
                       self.name, self.species, self.water_frequency)
            }
            
            // Private method
            fn needs_water(&self) -> bool {
                // Some private implementation
                true
            }
            
            // Public method that uses private method and field
            pub fn water_if_needed(&self) {
                if self.needs_water() {
                    println!("Watering {} every {} days", 
                             self.name, self.water_frequency);
                }
            }
        }
        
        // Private struct - only usable within this module or child modules
        struct PlantCare {
            plant: Plant,
            last_watered: String,
        }
        
        impl PlantCare {
            // Even though the struct is private, we can still define public methods
            // But they're only public within this module
            pub fn new(plant: Plant) -> PlantCare {
                PlantCare {
                    plant,
                    last_watered: "2023-01-01".to_string(),
                }
            }
        }
        
        // Public function that uses a private struct
        pub fn create_plant_care(name: &str, species: &str) -> String {
            let plant = Plant::new(name, species, 7);
            let care = PlantCare::new(plant);
            
            format!("Created care plan for {} ({}). Last watered: {}", 
                   care.plant.name, care.plant.species, care.last_watered)
        }
    }
    
    // Create a plant using the public constructor
    let mut plant = plants::Plant::new("Venus Flytrap", "Dionaea muscipula", 3);
    
    // Access and modify public fields
    println!("Plant name: {}", plant.name);
    plant.name = "Modified Plant".to_string();
    println!("Modified plant name: {}", plant.name);
    
    // Use public methods
    println!("Plant description: {}", plant.description());
    plant.water_if_needed();
    
    // This would cause an error - can't access private field
    // println!("Water frequency: {}", plant.water_frequency);
    
    // This would cause an error - can't access private method
    // plant.needs_water();
    
    // This would cause an error - can't create private struct
    // let plant_care = plants::PlantCare::new(plant);
    
    // But we can use a public function that uses the private struct internally
    let care_plan = plants::create_plant_care("Cactus", "Opuntia");
    println!("Care plan: {}", care_plan);
    
    //------------------------------------------------------
    // ENUM PRIVACY
    //------------------------------------------------------
    println!("\n=== Enum Privacy ===");
    
    mod plant_types {
        // Public enum - all variants are automatically public
        pub enum PlantType {
            Flowering,
            NonFlowering,
            Fern,
            Moss,
        }
        
        // We can still have a private enum
        enum GrowthRate {
            Slow,
            Medium,
            Fast,
        }
        
        // Public function that uses private enum
        pub fn describe_growth(plant_type: PlantType) -> &'static str {
            let growth = match plant_type {
                PlantType::Flowering => GrowthRate::Medium,
                PlantType::NonFlowering => GrowthRate::Slow,
                PlantType::Fern => GrowthRate::Fast,
                PlantType::Moss => GrowthRate::Slow,
            };
            
            match growth {
                GrowthRate::Slow => "This plant grows slowly",
                GrowthRate::Medium => "This plant grows at a medium rate",
                GrowthRate::Fast => "This plant grows quickly",
            }
        }
    }
    
    // Use the public enum and its variants
    let plant_type = plant_types::PlantType::Fern;
    println!("Growth description: {}", plant_types::describe_growth(plant_type));
    
    // This would cause an error - can't use private enum
    // let growth = plant_types::GrowthRate::Fast;
    
    //------------------------------------------------------
    // MODULE PRIVACY
    //------------------------------------------------------
    println!("\n=== Module Privacy ===");
    
    mod garden_center {
        // Public nested module
        pub mod plants {
            pub fn sell_plant() {
                println!("Selling a plant!");
            }
        }
        
        // Private nested module
        mod inventory {
            pub fn add_to_inventory() {
                println!("Adding to inventory!");
            }
        }
        
        // Public function that uses private module
        pub fn restock() {
            println!("Restocking the garden center!");
            inventory::add_to_inventory();
        }
    }
    
    // Can access public function in public module
    garden_center::plants::sell_plant();
    
    // Can access public function that uses private module
    garden_center::restock();
    
    // This would cause an error - can't access private module
    // garden_center::inventory::add_to_inventory();
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Privacy Design
//
// Create a module for a library system with appropriate privacy settings.
// Requirements:
// - Books should have public title and author, but private ISBN
// - Library should be able to add and find books
// - Users should be able to check out books but not modify the library's records directly

mod challenge_1_library_system {
    // TODO: Implement the library system with appropriate privacy settings
    
    // Hint: Something like this structure:
    // pub struct Book { ... }
    // pub struct Library { ... }
    
    // Implementation for Book
    // impl Book { ... }
    
    // Implementation for Library
    // impl Library { ... }
}

// Challenge 2: Module Hierarchy Privacy
//
// Fix the privacy modifiers in this module hierarchy to make the
// code in the test function work correctly.

mod challenge_2_privacy {
    // This module simulates a simple banking system
    
    mod bank {
        mod accounts {
            struct Account {
                id: u32,
                owner: String,
                balance: f64,
            }
            
            impl Account {
                fn new(id: u32, owner: String, initial_balance: f64) -> Account {
                    Account {
                        id,
                        owner,
                        balance: initial_balance,
                    }
                }
                
                fn deposit(&mut self, amount: f64) {
                    self.balance += amount;
                }
                
                fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
                    if amount <= self.balance {
                        self.balance -= amount;
                        Ok(())
                    } else {
                        Err("Insufficient funds")
                    }
                }
                
                fn get_balance(&self) -> f64 {
                    self.balance
                }
            }
        }
        
        mod transactions {
            fn record_transaction(account_id: u32, amount: f64, transaction_type: &str) {
                println!("Transaction recorded: {} ${} for account #{}", 
                         transaction_type, amount, account_id);
            }
        }
        
        // Customer-facing API
        mod customer_api {
            fn open_account(owner: &str, initial_deposit: f64) -> u32 {
                // Create a new account
                let account_id = 12345; // In a real system, this would be generated
                
                // Record the transaction
                super::transactions::record_transaction(
                    account_id, initial_deposit, "initial deposit");
                
                account_id
            }
            
            fn deposit(account_id: u32, amount: f64) {
                super::transactions::record_transaction(
                    account_id, amount, "deposit");
            }
            
            fn withdraw(account_id: u32, amount: f64) -> Result<(), &'static str> {
                super::transactions::record_transaction(
                    account_id, amount, "withdrawal");
                
                // In a real system, this would actually modify the account
                Ok(())
            }
            
            fn get_balance(account_id: u32) -> f64 {
                // In a real system, this would look up the account
                5000.0
            }
        }
    }
    
    // This function tests our banking system
    // Fix the privacy modifiers to make this function work
    pub fn test_bank() -> Result<(), String> {
        // Create a new account
        let account_id = bank::customer_api::open_account("John Doe", 1000.0);
        
        // Make some transactions
        bank::customer_api::deposit(account_id, 500.0);
        
        if let Err(e) = bank::customer_api::withdraw(account_id, 200.0) {
            return Err(format!("Withdrawal failed: {}", e));
        }
        
        // Check the balance
        let balance = bank::customer_api::get_balance(account_id);
        println!("Final balance: ${}", balance);
        
        // We expect the balance to be $1300 ($1000 initial + $500 deposit - $200 withdrawal)
        if balance != 1300.0 {
            return Err(format!("Expected balance to be $1300, got ${}", balance));
        }
        
        Ok(())
    }
}

// Challenge 3: Struct Field Privacy
//
// Modify the privacy settings of this Document struct and its methods
// to enforce these rules:
// - The content of a document can be read but not directly modified
// - The version can be read but not modified at all
// - Documents can be created and updated only through controlled methods

mod challenge_3_document {
    struct Document {
        title: String,
        content: String,
        version: u32,
    }
    
    impl Document {
        fn new(title: String, content: String) -> Document {
            Document {
                title,
                content,
                version: 1,
            }
        }
        
        fn update_content(&mut self, new_content: String) {
            self.content = new_content;
            self.version += 1;
        }
        
        fn get_content(&self) -> &str {
            &self.content
        }
        
        fn get_version(&self) -> u32 {
            self.version
        }
    }
    
    // This function tests our Document implementation
    // Fix the privacy modifiers to make this function work
    pub fn test_document() -> Result<(), String> {
        // Create a new document
        let mut doc = Document::new(
            "Privacy in Rust".to_string(),
            "Rust has a sophisticated privacy system...".to_string()
        );
        
        // Read the initial content and version
        let initial_content = doc.get_content();
        let initial_version = doc.get_version();
        
        println!("Initial document (v{}): {}", initial_version, initial_content);
        
        // Update the content through the controlled method
        doc.update_content("Rust's privacy system is based on modules...".to_string());
        
        // Verify the version incremented
        let new_version = doc.get_version();
        if new_version != 2 {
            return Err(format!("Expected version 2, got {}", new_version));
        }
        
        // These lines should cause compiler errors due to privacy restrictions
        // Uncomment them to test your implementation
        // doc.content = "Directly changing content".to_string();
        // doc.version = 10;
        
        Ok(())
    }
}

// A function to run all challenges
pub fn run_privacy_challenges() {
    println!("\n=== Running Privacy Challenges ===");
    
    if let Err(e) = challenge_2_privacy::test_bank() {
        println!("Challenge 2 failed: {}", e);
    } else {
        println!("Challenge 2 succeeded!");
    }
    
    if let Err(e) = challenge_3_document::test_document() {
        println!("Challenge 3 failed: {}", e);
    } else {
        println!("Challenge 3 succeeded!");
    }
} 