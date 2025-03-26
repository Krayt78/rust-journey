// Function Parameters in Rust
//
// This program demonstrates how to define functions with parameters
// and pass arguments to them.

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in these functions to make them compile
//------------------------------------------------------

// TODO: Fix this function by adding the missing parameter type
fn multiply(a: i32, b) {
    println!("{} * {} = {}", a, b, a * b);
}

// TODO: Fix this function to take two parameters: name and age
fn describe_person() {
    println!("{} is {} years old", name, age);
}

// TODO: Fix this function that should print a greeting message
// from one person to another
fn send_greeting(to: &str) {
    println!("From {}: Hello, {}!", to, from);
}

fn main() {
}