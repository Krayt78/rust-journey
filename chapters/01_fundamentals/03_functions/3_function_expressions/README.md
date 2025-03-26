# Function Expressions in Rust

Rust treats functions in a unique way compared to many other programming languages, making heavy use of expressions for return values rather than explicit return statements.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) in the official Rust Book, specifically the parts about expressions.

## Key Concepts

- **Statements vs Expressions**: Understanding the difference between statements (which don't return values) and expressions (which do)
- **Implicit Returns**: Learning how Rust returns the value of the last expression in a function
- **Conditional Expressions**: Using `if` expressions to return different values based on conditions
- **Block Expressions**: Working with code blocks as expressions that return values
- **Idiomatic Returns**: Writing Rust code that uses expression-based returns rather than explicit `return` statements

## Learning Materials

This directory contains several resources to help you learn about function expressions in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with challenges related to function expressions
2. [`learnings.md`](./learnings.md) - Detailed explanations about expressions in Rust functions

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

The file contains three function expression-related challenges to help you practice:

1. **Unnecessary Return Statements**: Fix the `absolute_value` function to use expression-based returns instead of explicit `return` statements
2. **Missing Return Values**: Fix the `grade_message` function that has semicolons after expressions that should be returned
3. **Block Expression Issues**: Fix the `factorial` function that has a syntax error in its block expression

For each challenge:
1. Read the TODO comments for instructions
2. Make the necessary changes to fix the code
3. Compile and run the program to check if your solution works

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the examples of how expressions are used in different contexts
3. Experiment by modifying the examples and creating your own function expressions

## What's Next?

Once you understand function expressions, you'll be ready to learn about more advanced function features in Rust, such as higher-order functions, closures, and function pointers. 