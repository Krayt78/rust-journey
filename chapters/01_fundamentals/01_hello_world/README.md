# Getting Started with Rust: Hello, World!

The traditional first program in any language is "Hello, World!", and Rust is no exception. This simple program demonstrates the basic structure of a Rust program and lets us confirm that our Rust installation is working correctly.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 1.2: Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html) in the official Rust Book.

## Key Concepts

- **Basic program structure**: Understanding the `main` function
- **Output**: Using the `println!` macro
- **Macros**: Introduction to Rust's macro system
- **Compilation**: How to compile and run Rust code

## The Hello World Program

In the file [`hello.rs`](./hello.rs), you'll find a comprehensive "Hello, World!" program with examples and challenges.

```rust
fn main() {
    println!("Hello, World!");
}
```

Let's break down what's happening here:

1. `fn main()`: This declares a function named `main`. The `main` function is special - it's the entry point of every Rust program.

2. `println!("Hello, World!")`: This is a call to the `println!` macro, which prints text to the console. Note the exclamation mark (`!`) - this indicates that `println!` is a macro, not a regular function.

## Running Your First Rust Program

To compile and run this program:

1. Open a terminal and navigate to this directory
2. Run the following command:

```bash
rustc hello.rs
```

3. This creates an executable file. To run it:

```bash
# On Unix-like systems:
./hello

# On Windows:
.\hello.exe
```

You should see the output of the program printed to your terminal.

## Learning Through Challenges

Each file in this course has two parts:

1. **Teaching Section**: The top part of each file contains working examples that demonstrate concepts with explanations in comments.

2. **Challenges Module**: A module at the bottom of each file with intentionally broken code that you need to fix. These challenges test your understanding of the concepts presented in the teaching section.

To complete the challenges:
1. Read through the teaching section to understand the concepts
2. Look at the broken code in the challenges module
3. Fix each issue to make the code compile and run correctly
4. The code contains validations that will tell you if your fixes are correct

In the `hello.rs` file, you'll find several challenges related to printing:
- Fixing a broken print statement
- Printing with variables
- Formatting output with multiple values

## Your First Challenge

1. Open `hello.rs` and find the `challenges` module
2. Fix the broken `printline!` macro to make it compile
3. Update the remaining functions to pass all the tests
4. Run the program to verify your solutions

## What's Next?

Now that you've created your first Rust program and completed your first challenges, you're ready to learn about variables and data types, which are covered in the next section.