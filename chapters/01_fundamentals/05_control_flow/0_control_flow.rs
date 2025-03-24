// Control Flow in Rust
//
// This program demonstrates how control flow works in Rust, including:
// - If expressions
// - Loops (loop, while, for)
// - Match expressions
// - If let and while let patterns
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    println!("Exploring Control Flow in Rust!");
    
    //------------------------------------------------------
    // IF EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- If Expressions ---");
    
    let number = 5;
    
    if number < 0 {
        println!("The number is negative");
    } else if number == 0 {
        println!("The number is zero");
    } else {
        println!("The number is positive");
    }
    
    // If expressions with return values
    let condition = true;
    let value = if condition { "condition is true" } else { "condition is false" };
    println!("The value is: {}", value);
    
    //------------------------------------------------------
    // LOOP EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- Loop Expression ---");
    
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // Return a value from the loop
        }
    };
    
    println!("The result of the loop is: {}", result);
    
    //------------------------------------------------------
    // WHILE LOOPS
    //------------------------------------------------------
    println!("\n--- While Loop ---");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
    
    //------------------------------------------------------
    // FOR LOOPS
    //------------------------------------------------------
    println!("\n--- For Loop with Array ---");
    
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    println!("\n--- For Loop with Range ---");
    
    for number in 1..4 {
        println!("{}!", number);
    }
    
    println!("\n--- For Loop with Enumerate ---");
    
    let names = vec!["Alice", "Bob", "Charlie"];
    
    for (index, name) in names.iter().enumerate() {
        println!("{} is at index {}", name, index);
    }
    
    //------------------------------------------------------
    // MATCH EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- Match Expression ---");
    
    let dice_roll = 4;
    
    match dice_roll {
        1 => println!("You rolled a one!"),
        2 => println!("You rolled a two!"),
        3 => println!("You rolled a three!"),
        4..=6 => println!("You rolled between four and six: {}", dice_roll),
        _ => println!("Invalid dice roll"),
    }
    
    // Match with binding
    println!("\n--- Match with Binding ---");
    
    let maybe_value: Option<i32> = Some(42);
    
    match maybe_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no value"),
    }
    
    //------------------------------------------------------
    // IF LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- If Let Expression ---");
    
    let some_value: Option<i32> = Some(3);
    
    if let Some(value) = some_value {
        println!("The value is: {}", value);
    } else {
        println!("No value");
    }
    
    //------------------------------------------------------
    // WHILE LET EXPRESSIONS
    //------------------------------------------------------
    println!("\n--- While Let Expression ---");
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the if expression
    pub fn challenge_if_expression() -> Result<(), String> {
        let number = 7;
        
        // This if expression should check if number is greater than 5
        // but the condition is wrong. Fix it.
        let message = if number < 5 {
            "number is greater than 5"
        } else {
            "number is 5 or less"
        };
        
        if message != "number is greater than 5" {
            return Err("The message should indicate number is greater than 5".to_string());
        }
        
        Ok(())
    }
    
    // TODO: Fix the loop to correctly count up to 5
    pub fn challenge_loop() -> Result<(), String> {
        let mut counter = 0;
        let result;
        
        // This loop should count up to 5 and return counter * 2
        // but there's an issue with the break condition
        result = loop {
            counter += 1;
            
            if counter == 3 {
                break counter * 2;
            }
        };
        
        if result != 10 {
            return Err(format!("The result should be 10, but got {}", result));
        }
        
        Ok(())
    }
    
    // TODO: Fix the while loop to print numbers from 1 to 3
    pub fn challenge_while_loop() -> Result<(), String> {
        let mut counter = 3;
        let mut output = String::new();
        
        // This while loop should count from 1 to 3, but it's counting down
        // Fix it to count up from 1 to 3
        while counter > 0 {
            output.push_str(&format!("{}", counter));
            counter -= 1;
        }
        
        if output != "123" {
            return Err(format!("Output should be '123', but got '{}'", output));
        }
        
        Ok(())
    }
    
    // TODO: Fix the for loop to properly iterate over the elements
    pub fn challenge_for_loop() -> Result<(), String> {
        let numbers = [10, 20, 30, 40, 50];
        let mut sum = 0;
        
        // This for loop should add up all the numbers, but there's an issue
        // Fix it to correctly sum all values
        for i in 0..3 {
            sum += numbers[i];
        }
        
        if sum != 150 {
            return Err(format!("Sum should be 150, but got {}", sum));
        }
        
        Ok(())
    }
    
    // TODO: Fix the match expression to handle all cases correctly
    pub fn challenge_match() -> Result<(), String> {
        let value = 42;
        let description;
        
        // This match expression should categorize the number
        // but it's missing cases and has a logic error
        match value {
            1 => description = "one",
            2 => description = "two",
            // Fix the match expression to correctly categorize 42
            _ => description = "small number",
        }
        
        if description != "large number" {
            return Err(format!("42 should be described as 'large number', but got '{}'", description));
        }
        
        Ok(())
    }
    
    // TODO: Fix the if let expression to extract the value correctly
    pub fn challenge_if_let() -> Result<(), String> {
        let optional: Option<i32> = Some(42);
        let mut value = 0;
        
        // This if let should extract the value from optional and multiply it by 2
        // Fix it to correctly extract and transform the value
        if let None = optional {
            value = 84;
        }
        
        if value != 84 {
            return Err(format!("Value should be 84, but got {}", value));
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
        println!("\nRunning Control Flow challenges...");
        
        // Challenge 1: Fix if expression
        if let Err(e) = challenges::challenge_if_expression() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the if expression!");
        
        // Challenge 2: Fix loop
        if let Err(e) = challenges::challenge_loop() {
            return Err(format!("Challenge 2 failed: {}", e));
        }
        println!("Successfully fixed the loop!");
        
        // Challenge 3: Fix while loop
        if let Err(e) = challenges::challenge_while_loop() {
            return Err(format!("Challenge 3 failed: {}", e));
        }
        println!("Successfully fixed the while loop!");
        
        // Challenge 4: Fix for loop
        if let Err(e) = challenges::challenge_for_loop() {
            return Err(format!("Challenge 4 failed: {}", e));
        }
        println!("Successfully fixed the for loop!");
        
        // Challenge 5: Fix match expression
        if let Err(e) = challenges::challenge_match() {
            return Err(format!("Challenge 5 failed: {}", e));
        }
        println!("Successfully fixed the match expression!");
        
        // Challenge 6: Fix if let expression
        if let Err(e) = challenges::challenge_if_let() {
            return Err(format!("Challenge 6 failed: {}", e));
        }
        println!("Successfully fixed the if let expression!");
        
        println!("All Control Flow challenges completed successfully!");
        Ok(())
    }
} 