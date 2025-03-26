# Hello World in Rust

This document explains the basic structure of a Rust program and how to output text to the console.

For more information, see [The Rust Book: Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

## Program Structure

The main function is the entry point of every Rust program. When you run a Rust program, execution begins in the main function:

```rust
fn main() {
    // Your code goes here
}
```

## Basic Printing

`println!` is a macro (not a function) that prints text to the console. Macros are denoted with an exclamation mark (!):

```rust
println!("Hello, World!");
```

## Printing with Variables

You can include variables and expressions in `println!` using curly braces `{}`:

```rust
let name = "Rustacean";
println!("Hello, {}!", name);
```

## Multiple Values

You can also include multiple values:

```rust
let language = "Rust";
let year = 2025;
println!("I'm learning {} in {}!", language, year);
```

## Positional Parameters

You can specify which value goes where using numbers:

```rust
let language = "Rust";
let year = 2025;
println!("In {1}, I'm learning {0}!", language, year);
```

## Named Parameters

Named parameters work too:

```rust
let language = "Rust";
let year = 2025;
println!("I'm learning {language} in {year}!");
```

## Practice Challenges

For practice, try these challenges:

1. Fix a broken `println!` macro invocation
2. Print a message that includes a variable
3. Fix code to print values in the correct order

You can find these challenges in the `challenge.rs` file, along with tests to verify your solutions. 