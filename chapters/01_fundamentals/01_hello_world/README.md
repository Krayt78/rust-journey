# Getting Started with Rust: Hello, World!

The traditional first program in any language is "Hello, World!", and Rust is no exception. This simple program demonstrates the basic structure of a Rust program and lets us confirm that our Rust installation is working correctly.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 1.2: Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html) in the official Rust Book.

## Key Concepts

- **Basic program structure**: Understanding the `main` function
- **Output**: Using the `println!` macro
- **Compilation**: How to compile and run Rust code

## Learning Materials

This directory contains several resources to help you learn about Hello World in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with examples and challenges related to printing text
2. [`learnings.md`](./learnings.md) - Detailed explanations about printing in Rust

## Running the Code

### Compiling and Running the Example

To compile and run the example code in `challenge.rs`:

1. Open a terminal and navigate to this directory
2. Compile the code with rustc:
   ```bash
   rustc challenge.rs
   ```
3. Run the resulting executable:
   ```bash
   # On Unix-like systems:
   ./challenge

   # On Windows:
   .\challenge.exe
   ```
4. You should see the output of the program printed to your terminal

### Running the Tests

The challenges in `challenge.rs` include tests to verify your solutions. To run these tests:

1. Make sure you have fixed the code in the challenges module
2. Compile and run the program with:
   ```bash
   rustc challenge.rs && ./challenge
   ```
   (Use `.\challenge.exe` on Windows)
3. The program will automatically run the tests and report whether your fixes were successful

## Working with the Challenges

1. Open `challenge.rs` in your editor
2. Read through the examples at the top to understand the concepts
3. Find the `challenges` module (around line 40)
4. Fix each issue in the challenge functions:
   - `broken_print()`: Fix the println! macro invocation
   - `print_with_variable()`: Make it print "Hello, Rust!" using the name variable
   - `print_multiple_values()`: Fix to print values in the requested order
5. Run the program to verify your solutions pass all the tests

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the working examples in the main function of [`challenge.rs`](./challenge.rs)
3. Experiment by modifying the examples and observing the results

## What's Next?

Now that you've created your first Rust program and completed your first challenges, you're ready to learn about variables and data types, which are covered in the next section.