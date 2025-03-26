# Function Parameters in Rust

Function parameters allow you to create more flexible and reusable functions by accepting input data that can change each time the function is called.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.3: Functions - Parameters](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#parameters) in the official Rust Book.

## Key Concepts

- **Parameter Definition**: Defining parameters with names and types in function signatures
- **Type Annotations**: Understanding the requirement for explicit parameter types
- **Multiple Parameters**: Working with functions that accept multiple inputs
- **Parameter Order**: How the order of parameters affects function behavior

## Learning Materials

This directory contains several resources to help you learn about function parameters in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with challenges related to function parameters
2. [`learnings.md`](./learnings.md) - Detailed explanations about function parameters in Rust

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

The file contains three function parameter-related challenges to help you practice:

1. **Missing Type Annotation**: Fix the `multiply` function by adding the missing parameter type
2. **Missing Parameters**: Fix the `describe_person` function to take two parameters
3. **Parameter Order Issues**: Fix the `send_greeting` function that has incorrect parameter usage

For each challenge:
1. Read the TODO comments for instructions
2. Make the necessary changes to fix the code
3. Compile and run the program to check if your solution works

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the examples of functions with different parameter patterns
3. Experiment by modifying the examples and creating your own functions with parameters

## What's Next?

Once you understand function parameters, you'll be ready to learn about function return values and how to use functions more effectively in your Rust programs. 