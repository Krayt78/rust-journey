# Function Return Values in Rust

Return values allow functions to compute and provide results back to the code that called them, making functions more useful and reusable.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.3: Functions - Return Values](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values) in the official Rust Book.

## Key Concepts

- **Return Type Syntax**: Specifying return types with the `->` syntax
- **Implicit Returns**: Understanding how the last expression in a function is returned
- **Unit Type**: Working with functions that don't return meaningful values
- **Multiple Return Values**: Returning multiple values using tuples
- **Early Returns**: Using the `return` keyword for conditional logic

## Learning Materials

This directory contains several resources to help you learn about function return values in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with challenges related to function return values
2. [`learnings.md`](./learnings.md) - Detailed explanations about return values in Rust

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

The file contains several function return value-related challenges to help you practice:

1. **Missing Return Value**: Fix the `square` function to properly return the square of a number
2. **Redundant Return Statements**: Fix the `is_even` function that has unnecessary return statements
3. **Tuple Return Syntax**: Fix the `calculate` function that returns a tuple incorrectly
4. **Incomplete Code Paths**: Fix the `max_of_three` function to handle all code paths

For each challenge:
1. Read the TODO comments for instructions
2. Make the necessary changes to fix the code
3. Compile and run the program to check if your solution works

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the examples of functions with different return value patterns
3. Experiment by modifying the examples and creating your own functions with return values

## What's Next?

Once you understand function return values, you'll be ready to combine your knowledge of parameters and return values to build more complex and useful functions in Rust. 