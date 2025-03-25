// Enums in Rust
//
// This program demonstrates how enums work in Rust, including:
// - Basic enums with different variants
// - Enums with associated data
// - The Option enum for nullable values
// - Pattern matching with enums
// - Methods on enums
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch06-00-enums.html

fn main() {
    println!("Exploring Enums in Rust!");
    
    //------------------------------------------------------
    // BASIC ENUMS
    //------------------------------------------------------
    println!("\n--- Basic Enums ---");
    
    // Define a simple enum
    enum IpAddrKind {
        V4,
        V6,
    }
    
    // Create instances of enum variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // We can define functions that take enums
    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("Routing IPv4..."),
            IpAddrKind::V6 => println!("Routing IPv6..."),
        }
    }
    
    route(four);
    route(six);
    
    //------------------------------------------------------
    // ENUMS WITH DATA
    //------------------------------------------------------
    println!("\n--- Enums with Data ---");
    
    // Enum with data in its variants
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    fn describe_ip(ip: IpAddr) {
        match ip {
            IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
        }
    }
    
    describe_ip(home);
    describe_ip(loopback);
    
    //------------------------------------------------------
    // COMPLEX ENUM VARIANTS
    //------------------------------------------------------
    println!("\n--- Complex Enum Variants ---");
    
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Named fields like a struct
        Write(String),              // String data
        ChangeColor(i32, i32, i32), // Three i32 values
    }
    
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to position ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB: ({}, {}, {})", r, g, b),
        }
    }
    
    process_message(Message::Quit);
    process_message(Message::Move { x: 10, y: 20 });
    process_message(Message::Write(String::from("Hello, Rust!")));
    process_message(Message::ChangeColor(255, 0, 255));
    
    //------------------------------------------------------
    // THE OPTION ENUM
    //------------------------------------------------------
    println!("\n--- The Option Enum ---");
    
    // The Option enum is defined by the standard library:
    //
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    
    let some_number = Some(5);
    let some_string = Some("a string");
    
    // If we use None, we need to tell Rust what type the Option is
    let absent_number: Option<i32> = None;
    
    // Using Option with unwrap
    println!("some_number unwrapped: {}", some_number.unwrap());
    println!("some_string unwrapped: {}", some_string.unwrap());
    
    // This would panic: println!("absent_number unwrapped: {}", absent_number.unwrap());
    
    // A safer way is to use pattern matching
    match absent_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number provided"),
    }
    
    //------------------------------------------------------
    // PATTERN MATCHING WITH OPTION
    //------------------------------------------------------
    println!("\n--- Pattern Matching with Option ---");
    
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    
    //------------------------------------------------------
    // METHODS ON ENUMS
    //------------------------------------------------------
    println!("\n--- Methods on Enums ---");
    
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Method called on Quit variant"),
                Message::Move { x, y } => println!("Method called on Move variant at ({}, {})", x, y),
                Message::Write(text) => println!("Method called on Write variant with text: {}", text),
                Message::ChangeColor(r, g, b) => println!("Method called on ChangeColor variant with RGB: ({}, {}, {})", r, g, b),
            }
        }
    }
    
    let msg = Message::Write(String::from("hello"));
    msg.call();
    
    //------------------------------------------------------
    // THE IF LET SYNTAX
    //------------------------------------------------------
    println!("\n--- The if let Syntax ---");
    
    // When we only care about one pattern and want to ignore the rest
    let some_value = Some(3);
    
    // With match
    match some_value {
        Some(3) => println!("three!"),
        _ => (),
    }
    
    // Same thing with if let (more concise)
    if let Some(3) = some_value {
        println!("three!");
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // Challenge 1: Fix this enum definition and usage
    pub fn challenge_basic_enum() -> Result<(), String> {
        // This enum definition has issues
        enum Direction {
            Up
            Down
            Left
            Right
        }
        
        // Fix this function to properly use the enum
        fn get_direction(index: usize) -> Direction {
            match index {
                0 => Direction.Up,
                1 => Direction.Down,
                2 => Direction.Left,
                3 => Direction.Right,
                _ => Direction.Up,
            }
        }
        
        // This should return the Left variant
        let dir = get_direction(2);
        
        // This comparison is not correct
        if dir != Direction::Left {
            return Err("Direction should be Left".to_string());
        }
        
        Ok(())
    }
    
    // Challenge 2: Fix this enum with data and pattern matching
    pub fn challenge_enum_with_data() -> Result<(), String> {
        // This enum has data in each variant, but there are issues
        enum Shape {
            Circle(f64),             // Radius
            Rectangle(f64, f64)      // Width, Height
            Triangle(f64, f64, f64), // Three sides
        }
        
        // Fix this function to calculate the area of each shape
        // Circle: π * r²
        // Rectangle: width * height
        // Triangle: Use Heron's formula
        fn calculate_area(shape: Shape) -> f64 {
            return match shape {
                Circle(r) => 3.14159 * r * r,
                Rectangle(w, h) => w + h,
                Shape::Triangle(a, b, c) {
                    let s = (a + b + c) / 2.0;
                    (s * (s - a) * (s - b) * (s - c)).sqrt()
                }
            }
        }
        
        let circle = Shape::Circle(2.0);
        let rectangle = Shape::Rectangle(3.0, 4.0);
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        
        let circle_area = calculate_area(circle);
        let rectangle_area = calculate_area(rectangle);
        let triangle_area = calculate_area(triangle);
        
        if (circle_area - 12.56636).abs() > 0.01 ||
           (rectangle_area - 12.0).abs() > 0.01 ||
           (triangle_area - 6.0).abs() > 0.01 {
            return Err(format!("Incorrect area calculations. Circle: {}, Rectangle: {}, Triangle: {}", 
                        circle_area, rectangle_area, triangle_area));
        }
        
        Ok(())
    }
    
    // Challenge 3: Fix the Option handling
    pub fn challenge_option_handling() -> Result<(), String> {
        // Fix this function to safely handle Option values
        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                numerator / denominator
            }
        }
        
        let result1 = divide(10.0, 2.0);
        let result2 = divide(5.0, 0.0);
        
        // This code should safely extract the value from result1
        // and provide a default value for result2
        let value1 = result1;
        let value2 = result2;
        
        if value1 != 5.0 || value2 != 0.0 {
            return Err(format!("Incorrect values. value1: {:?}, value2: {:?}", value1, value2));
        }
        
        Ok(())
    }
    
    // Challenge 4: Implement methods for an enum
    pub fn challenge_enum_methods() -> Result<(), String> {
        // Implement a method for this enum to convert it to an integer value
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        
        // The code as written won't compile - implement the required method
        let penny = Coin::Penny;
        let nickel = Coin::Nickel;
        let dime = Coin::Dime;
        let quarter = Coin::Quarter;
        
        let penny_value = penny.value_in_cents();
        let nickel_value = nickel.value_in_cents();
        let dime_value = dime.value_in_cents();
        let quarter_value = quarter.value_in_cents();
        
        if penny_value != 1 || nickel_value != 5 || dime_value != 10 || quarter_value != 25 {
            return Err(format!("Incorrect coin values. Penny: {}, Nickel: {}, Dime: {}, Quarter: {}", 
                        penny_value, nickel_value, dime_value, quarter_value));
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
        println!("\nRunning Enums challenges...");
        
        // Challenge 1: Fix basic enum
        if let Err(e) = challenges::challenge_basic_enum() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the basic enum issue!");
        
        // Challenge 2: Fix enum with data
        if let Err(e) = challenges::challenge_enum_with_data() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the enum with data issue!");
        
        // Challenge 3: Fix option handling
        if let Err(e) = challenges::challenge_option_handling() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the option handling issue!");
        
        // Challenge 4: Implement enum methods
        if let Err(e) = challenges::challenge_enum_methods() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully implemented the enum methods!");
        
        println!("All Enums challenges completed successfully!");
        Ok(())
    }
} 