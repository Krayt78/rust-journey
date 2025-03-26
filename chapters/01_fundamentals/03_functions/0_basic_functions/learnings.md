# Basic Functions in Rust

This document explains the fundamental concepts of functions in Rust.

For more information, see [The Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Function Basics

Functions are defined using the `fn` keyword followed by a name and a pair of parentheses. The function body is enclosed in curly braces:

```rust
fn say_hello() {
    println!("Hello from the say_hello function!");
}
```

## Calling Functions

Functions are called by using their name followed by parentheses:

```rust
fn main() {
    // Calling a function
    say_hello();
    
    // Functions can be called multiple times
    say_hello();
    say_hello();
    
    // The code runs sequentially - this prints after the function calls
    println!("Back in the main function!");
}

fn say_hello() {
    println!("Hello from the say_hello function!");
}
```

## Function Structure

A function can contain multiple statements and have its own local variables:

```rust
fn example_function() {
    // This is the function body
    // It contains all the code that runs when the function is called
    println!("This is an example function!");
    
    // Functions can contain multiple statements
    let x = 5;
    println!("The value of x is: {}", x);
    
    // The function ends at the closing brace
}
```

## Function Declaration Location

Functions in Rust can be defined anywhere in the file:

- Before or after they are called
- At the top-level of the file (not nested inside other functions)

```rust
fn main() {
    say_hello(); // We can call this function even though it's defined below
}

fn say_hello() {
    println!("Hello from the say_hello function!");
}
```

This flexibility allows you to organize your code in a way that makes sense for your program.

## Function Execution Flow

Code in Rust executes sequentially, line by line:

1. When a function is called, execution jumps to the function's body
2. All statements in the function are executed in order
3. When the function ends, execution returns to the point right after the function call

This allows you to break your code into manageable, reusable pieces.

## Practice Challenges

To practice working with basic functions, try implementing:

1. A function that prints your name
2. A function that prints a custom message multiple times
3. Multiple functions that call each other in sequence

You can find challenges to practice these concepts in the `challenge.rs` file.
