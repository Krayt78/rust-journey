# Variables and Mutability in Rust

Variables are a fundamental concept in any programming language. In Rust, variables have some unique characteristics that help ensure memory safety and prevent common bugs.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) in the official Rust Book.

## Key Concepts

- **Immutability**: Variables are immutable by default
- **Mutability**: Using the `mut` keyword to make variables mutable
- **Shadowing**: Creating a new variable with the same name
- **Constants**: Defining values that never change

## Learning Materials

This directory contains several resources to help you learn about variables in Rust:

1. [`challenge.rs`](./challenge.rs) - A Rust program with challenges related to variables
2. [`learnings.md`](./learnings.md) - Detailed explanations about variables in Rust

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
4. You should see instructions for the challenges

### Working with the Challenges

The file contains several challenges that are commented out by default. To work with them:

1. Open `challenge.rs` in your editor
2. Uncomment each challenge section (remove the `/*` and `*/` markers)
3. Fix the code in each challenge to make it work
4. Run the program again to see if your fixes were successful

## Working with the Challenges

1. Open `challenge.rs` in your editor
2. You'll find several challenges to fix:
   - Fix a mutable variable declaration
   - Implement proper shadowing
   - Fix a constant declaration
   - Fix a complex shadowing example
   - Correct a boolean shadowing issue
3. For each challenge:
   - Uncomment the code section
   - Fix the issue based on the comment instructions
   - Run the program to check if your solution works
4. The program will show ✅ for successful solutions or ❌ if there's still an issue

## Learning the Concepts

To understand the concepts being demonstrated:

1. Read the detailed explanations in [`learnings.md`](./learnings.md)
2. Study the code examples in the main function of [`learnings.md`](./learnings.md)
3. Experiment by modifying the examples and observing the results

## What's Next?

Now that you understand variables, you're ready to learn about data types in Rust, which are covered in the next section.