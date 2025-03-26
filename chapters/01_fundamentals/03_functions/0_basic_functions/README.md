# Basic Functions in Rust

Functions are a fundamental building block in Rust programming. They allow you to organize your code into reusable and logical units.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) in the official Rust Book.

## Key Concepts

- **Function Declaration**: Using the `fn` keyword to define functions
- **Function Calls**: Invoking functions by name with parentheses
- **Function Structure**: Understanding the components of a function
- **Function Location**: Where functions can be placed in your code

## Learning Materials

This directory contains several resources to help you learn about functions in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with challenges related to basic functions
2. [`learnings.md`](./learnings.md) - Detailed explanations about functions in Rust

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

Note: The challenge file contains code that won't compile in its initial state. You'll need to fix the issues as part of the learning exercise.

## Working with the Challenges

The file contains several function-related challenges to help you practice:

1. Fix the `say_good_morning()` function by adding the `fn` keyword and correcting the syntax
2. Fix the `print_welcome` function that has multiple issues
3. Define a new function called `print_number` that prints the number 42
4. Call all three functions in the correct order within the `run_functions()` function

For each challenge:
1. Read the TODO comments for instructions
2. Make the necessary changes to fix the code
3. Compile and run the program to check if your solution works

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the code examples and understand how functions are defined and called
3. Experiment by modifying the examples and creating your own functions

## What's Next?

Once you understand basic functions, you'll be ready to learn about more advanced function features in Rust, such as functions with parameters and return values. 