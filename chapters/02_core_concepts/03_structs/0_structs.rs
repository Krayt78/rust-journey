// Structs in Rust
//
// This program demonstrates how structs work in Rust, including:
// - Basic structs with named fields
// - Tuple structs
// - Unit structs
// - Methods and associated functions
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch05-00-structs.html

fn main() {
    println!("Exploring Structs in Rust!");
    
    //------------------------------------------------------
    // BASIC STRUCTS
    //------------------------------------------------------
    println!("\n--- Basic Structs ---");
    
    // Define a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // Create an instance of the struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {} ({})", user1.username, user1.email);
    
    // Create a mutable instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anothername456"),
        active: true,
        sign_in_count: 1,
    };
    
    // Change a field
    user2.email = String::from("newemail@example.com");
    println!("Updated email: {}", user2.email);
    
    //------------------------------------------------------
    // CREATING INSTANCES FROM OTHER INSTANCES
    //------------------------------------------------------
    println!("\n--- Creating Instances from Other Instances ---");
    
    // Using struct update syntax
    let user3 = User {
        email: String::from("third@example.com"),
        ..user1 // All other fields come from user1
    };
    
    println!("New user: {} ({})", user3.username, user3.email);
    
    //------------------------------------------------------
    // TUPLE STRUCTS
    //------------------------------------------------------
    println!("\n--- Tuple Structs ---");
    
    // Define tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    //------------------------------------------------------
    // UNIT STRUCTS
    //------------------------------------------------------
    println!("\n--- Unit Structs ---");
    
    // A unit struct doesn't have any fields
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    
    // Unit structs are useful when you need to implement a trait on some
    // type but don't have any data to store in the type itself
    println!("Unit struct example: AlwaysEqual");
    
    //------------------------------------------------------
    // METHODS
    //------------------------------------------------------
    println!("\n--- Methods ---");
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // Method to calculate area
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // Method that takes another Rectangle parameter
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // Associated function (doesn't take self)
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1 is {:?}", rect1);
    println!("Area of rect1: {} square pixels", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    //------------------------------------------------------
    // ASSOCIATED FUNCTIONS
    //------------------------------------------------------
    println!("\n--- Associated Functions ---");
    
    let square = Rectangle::square(25);
    println!("Created square: {:?}", square);
    println!("Area of square: {} square pixels", square.area());
    
    //------------------------------------------------------
    // MULTIPLE IMPL BLOCKS
    //------------------------------------------------------
    println!("\n--- Multiple impl Blocks ---");
    
    impl Rectangle {
        // You can have multiple impl blocks for the same struct
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    
    println!("Is rect1 a square? {}", rect1.is_square());
    println!("Is square a square? {}", square.is_square());
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // Challenge 1: Fix this struct definition and instantiation
    pub fn challenge_basic_struct() -> Result<(), String> {
        // This struct definition and instantiation has issues
        struct Person {
            name: String,
            age: u8,
            active: bool,
        }
        
        // This code isn't creating a valid Person - fix it
        let person = Person {
            name: "Alice",
            age: 30,
            status: true,
        };
        
        if person.name != "Alice" || person.age != 30 || !person.active {
            return Err(format!("Person has incorrect values: {:?}", person));
        }
        
        Ok(())
    }
    
    // Challenge 2: Fix this code to use struct update syntax properly
    pub fn challenge_struct_update() -> Result<(), String> {
        #[derive(Debug)]
        struct Car {
            make: String,
            model: String,
            year: u32,
            color: String,
            mileage: u32,
        }
        
        let car1 = Car {
            make: String::from("Toyota"),
            model: String::from("Corolla"),
            year: 2020,
            color: String::from("Blue"),
            mileage: 10_000,
        };
        
        // Use struct update syntax to create car2 with the same make and model
        // but different year, color, and mileage
        let car2 = Car {
            year: 2022,
            color: String::from("Red"),
        };
        
        if car2.make != "Toyota" || car2.model != "Corolla" || 
           car2.year != 2022 || car2.color != "Red" || car2.mileage != 5_000 {
            return Err(format!("Car2 has incorrect values: {:?}", car2));
        }
        
        Ok(())
    }
    
    // Challenge 3: Implement a method for this struct
    pub fn challenge_methods() -> Result<(), String> {
        struct Circle {
            radius: f64,
        }
        
        // Implement Circle methods to calculate area and perimeter
        // Area = π * r²
        // Perimeter = 2 * π * r
        
        let circle = Circle { radius: 5.0 };
        
        // Uncomment and fix these lines
        // let area = circle.area();
        // let perimeter = circle.perimeter();
        
        // For the purpose of this test, use a simplified π value of 3.14
        let expected_area = 3.14 * 5.0 * 5.0;
        let expected_perimeter = 2.0 * 3.14 * 5.0;
        
        if (area - expected_area).abs() > 0.01 || (perimeter - expected_perimeter).abs() > 0.01 {
            return Err(format!("Incorrect calculations. Expected area: {}, got: {}. Expected perimeter: {}, got: {}", 
                        expected_area, area, expected_perimeter, perimeter));
        }
        
        Ok(())
    }
    
    // Challenge 4: Fix and use this tuple struct
    pub fn challenge_tuple_struct() -> Result<(), String> {
        // Fix this tuple struct definition and create instances
        struct RGB(u8, u8, u8);
        
        // Create RGB instances for red, green, and blue colors
        let red = RGB(255);
        let green = RGB(0, 0);
        let blue = RGB();
        
        if red.0 != 255 || red.1 != 0 || red.2 != 0 {
            return Err(format!("Red has incorrect values: ({}, {}, {})", red.0, red.1, red.2));
        }
        
        if green.0 != 0 || green.1 != 255 || green.2 != 0 {
            return Err(format!("Green has incorrect values: ({}, {}, {})", green.0, green.1, green.2));
        }
        
        if blue.0 != 0 || blue.1 != 0 || blue.2 != 255 {
            return Err(format!("Blue has incorrect values: ({}, {}, {})", blue.0, blue.1, blue.2));
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
        println!("\nRunning Structs challenges...");
        
        // Challenge 1: Fix basic struct
        if let Err(e) = challenges::challenge_basic_struct() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the basic struct issue!");
        
        // Challenge 2: Fix struct update syntax
        if let Err(e) = challenges::challenge_struct_update() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the struct update syntax issue!");
        
        // Challenge 3: Implement methods
        if let Err(e) = challenges::challenge_methods() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully implemented the Circle methods!");
        
        // Challenge 4: Fix tuple struct
        if let Err(e) = challenges::challenge_tuple_struct() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the tuple struct issue!");
        
        println!("All Structs challenges completed successfully!");
        Ok(())
    }
} 