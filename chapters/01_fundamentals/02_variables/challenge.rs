// Variables in Rust
//
// This program contains coding challenges related to variables in Rust.
// For detailed explanations and learning content, see the learnings.md file in this directory
// or refer to The Rust Book: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html


//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this function to make it compile
//------------------------------------------------------
fn main() {
    // TODO: Fix the mutable variable
    // This variable needs to be mutable
    let x = 5;
    x = 10;
    
    //------------------------------------------------------

    // TODO: Fix this shadowing example
    // We start with a string
    let y = "hello";
    
    // We want to get the length using shadowing
    y = 5;

    //------------------------------------------------------

    // TODO: Fix the constant declaration
    // Constants should use SCREAMING_SNAKE_CASE and have a type annotation
    const max_score: u32 = 100;
}