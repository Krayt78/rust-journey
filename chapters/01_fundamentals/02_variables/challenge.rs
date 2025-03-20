// Variables Challenge
// Fix the errors in this program so it compiles and runs correctly

fn main() {
    // Challenge 1: Fix this variable declaration
    let mut x = 5;
    x = 10;
    println!("x is: {}", x);
    
    // Challenge 2: Make this code work using shadowing
    let y = "hello";
    // We want to perform a calculation on y
    let y = y.len();
    println!("y is: {}", y);
    
    // Challenge 3: Define a constant for the maximum score
    // Using a proper naming convention
    const MAX_SCORE: i32 = 100;
    println!("The maximum score is: {}", MAX_SCORE);
    
    // Challenge 4: Make this work without changing the final println!
    let mut value = 10;
    let value_squared = value * value;
    let value = value_squared;
    println!("The squared value is: {}", value);
    
    // Bonus Challenge: Do not change these two lines
    let special = "Rust";
    let special = special.contains("us");
    // Add code below to properly print the value of special
    println!("Does 'Rust' contain 'us'? {}", special);
    
    println!("All challenges completed!");
}