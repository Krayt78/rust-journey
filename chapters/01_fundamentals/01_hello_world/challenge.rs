// This is a comment in Rust

//------------------------------------------------------
// Hello World in Rust
//------------------------------------------------------
//
// This program demonstrates the basic structure of a Rust program
// and how to output text to the console.
//
// For detailed explanations, see the learnings.md file in this directory
// or refer to The Rust Book: https://doc.rust-lang.org/book/ch01-02-hello-world.html


//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
fn main() {
    // TODO: Fix the println! macro invocation
    // Note: it's "println" not "printline"
    printline!("Hello, World!");
    
    // TODO: Make this function print "Hello, Rust!" using the name variable
    let name = "Rust";
    println!("Hello!"); // Should print Hello, Rust!
    
    // TODO: Fix this function to print the values in the requested order
    let language = "Rust";
    let year = 2023;
        
    // Should print "Started learning Rust in 2023!"
    println!("Started learning {year} in {language}!");
}