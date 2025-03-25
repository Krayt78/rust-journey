// Pattern Matching in Rust
//
// This program demonstrates how pattern matching works in Rust, including:
// - Match expressions with different pattern types
// - Destructuring complex data
// - Match guards
// - Binding with @ operator
// - Shorthand forms like if let and while let
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch18-00-patterns.html

fn main() {
    println!("Exploring Pattern Matching in Rust!");
    
    //------------------------------------------------------
    // MATCH EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- Basic Match Expressions ---");
    
    let number = 13;
    
    // Match is an expression, so it returns a value
    let message = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else", // The _ pattern matches any value
    };
    
    println!("The number is {}", message);
    
    //------------------------------------------------------
    // MATCHING MULTIPLE PATTERNS
    //------------------------------------------------------
    println!("\n--- Matching Multiple Patterns ---");
    
    let dice_roll = 4;
    
    match dice_roll {
        1 | 2 => println!("Not very lucky"),
        3..=5 => println!("Pretty good"), // Range pattern (inclusive)
        6 => println!("Lucky six!"),
        other => println!("Invalid dice roll: {}", other), // Also binds the value
    }
    
    //------------------------------------------------------
    // DESTRUCTURING TUPLES
    //------------------------------------------------------
    println!("\n--- Destructuring Tuples ---");
    
    let point = (3, 2);
    
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("X-axis at y={}", y),
        (x, 0) => println!("Y-axis at x={}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
    
    // Destructuring nested tuples
    let nested = (1, (2, 3));
    
    match nested {
        (a, (b, c)) => println!("Nested values: a={}, b={}, c={}", a, b, c),
    }
    
    //------------------------------------------------------
    // DESTRUCTURING STRUCTS
    //------------------------------------------------------
    println!("\n--- Destructuring Structs ---");
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y } => println!("At coordinate ({}, {})", x, y),
    }
    
    // You can also destructure a struct with shorthand
    let Point { x, y } = p;
    println!("Destructured point: x={}, y={}", x, y);
    
    //------------------------------------------------------
    // DESTRUCTURING ENUMS
    //------------------------------------------------------
    println!("\n--- Destructuring Enums ---");
    
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
    
    //------------------------------------------------------
    // MATCH GUARDS
    //------------------------------------------------------
    println!("\n--- Match Guards ---");
    
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater than or equal to five: {}", x),
        None => println!("None"),
    }
    
    // Match guards with multiple patterns
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"), // Match only if y is true
        _ => println!("no"),
    }
    
    //------------------------------------------------------
    // BINDING WITH @ OPERATOR
    //------------------------------------------------------
    println!("\n--- Binding with @ Operator ---");
    
    enum Number {
        Value(i32),
        Missing,
    }
    
    let number = Number::Value(5);
    
    match number {
        Number::Value(n @ 1..=5) => println!("Got a small number: {}", n),
        Number::Value(n @ 6..=10) => println!("Got a medium number: {}", n),
        Number::Value(n) => println!("Got some other number: {}", n),
        Number::Missing => println!("No number"),
    }
    
    //------------------------------------------------------
    // IGNORING VALUES
    //------------------------------------------------------
    println!("\n--- Ignoring Values ---");
    
    // Ignoring entire values
    let _unused = 5; // Using _ prefix for unused variables
    
    // Ignoring parts of a value
    let coordinates = (2, 3, 4);
    
    match coordinates {
        (x, _, z) => println!("x = {}, z = {}", x, z), // Ignoring y
    }
    
    // Ignoring remaining parts of a value
    let numbers = (1, 2, 3, 4, 5);
    
    match numbers {
        (first, .., last) => println!("First: {}, Last: {}", first, last),
    }
    
    //------------------------------------------------------
    // IF LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- If Let Expressions ---");
    
    let favorite_color: Option<&str> = Some("purple");
    
    // Verbose match expression
    match favorite_color {
        Some(color) => println!("Your favorite color is {}", color),
        None => println!("You don't have a favorite color"),
    }
    
    // Same thing with if let
    if let Some(color) = favorite_color {
        println!("Your favorite color is {}", color);
    } else {
        println!("You don't have a favorite color");
    }
    
    //------------------------------------------------------
    // WHILE LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- While Let Expressions ---");
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // While let loop: keep going until stack.pop() returns None
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // Challenge 1: Fix this match expression
    pub fn challenge_basic_match() -> Result<(), String> {
        let dice_roll = 6;
        
        // This match expression has issues - fix it to handle all cases
        let result = match dice_roll {
            1 => "Critical failure",
            2..=5 => "Keep playing",
        };
        
        if result != "Success!" {
            return Err(format!("Expected 'Success!', got '{}'", result));
        }
        
        Ok(())
    }
    
    // Challenge 2: Fix this destructuring code
    pub fn challenge_destructuring() -> Result<(), String> {
        struct Person {
            name: String,
            age: u32,
            favorite_color: Option<String>,
        }
        
        let person = Person {
            name: String::from("Alice"),
            age: 30,
            favorite_color: Some(String::from("Blue")),
        };
        
        // This destructuring pattern is incorrect - fix it
        let Person { name, favorite_color: color } = person;
        
        // Check that we destructured correctly
        if name != "Alice" || age != 30 || color != Some(String::from("Blue")) {
            return Err("Failed to destructure Person correctly".to_string());
        }
        
        Ok(())
    }
    
    // Challenge 3: Fix this match guard and binding
    pub fn challenge_match_guard() -> Result<(), String> {
        enum Temperature {
            Celsius(i32),
            Fahrenheit(i32),
        }
        
        let temp = Temperature::Celsius(25);
        
        // This match expression with guards has issues - fix it
        let description = match temp {
            Temperature::Celsius(c) if c < 20 => "cool",
            Temperature::Celsius(c) => c > 30 => "warm",
            Temperature::Fahrenheit(f) if f < 40 => "cool",
            Temperature::Fahrenheit(f) if f > 86 => "warm",
        };
        
        if description != "comfortable" {
            return Err(format!("Expected 'comfortable', got '{}'", description));
        }
        
        Ok(())
    }
    
    // Challenge 4: Fix this if let and while let code
    pub fn challenge_if_while_let() -> Result<(), String> {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        
        // This iterator code using if let and while let has issues - fix it
        let mut iter = numbers.iter();
        
        if let first = iter.next() {
            sum += first;
        }
        
        while iter.next() {
            let Some(val) = iter.next();
            sum += val;
        }
        
        if sum != 9 {
            return Err(format!("Expected sum 9, got {}", sum));
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
        println!("\nRunning Pattern Matching challenges...");
        
        // Challenge 1: Fix basic match expression
        if let Err(e) = challenges::challenge_basic_match() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the basic match issue!");
        
        // Challenge 2: Fix destructuring
        if let Err(e) = challenges::challenge_destructuring() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the destructuring issue!");
        
        // Challenge 3: Fix match guard and binding
        if let Err(e) = challenges::challenge_match_guard() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the match guard issue!");
        
        // Challenge 4: Fix if let and while let
        if let Err(e) = challenges::challenge_if_while_let() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the if let and while let issue!");
        
        println!("All Pattern Matching challenges completed successfully!");
        Ok(())
    }
} 