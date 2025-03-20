# Getting Started with Rust: Hello, World!

The traditional first program in any language is "Hello, World!", and Rust is no exception. This simple program demonstrates the basic structure of a Rust program and lets us confirm that our Rust installation is working correctly.

## Key Concepts

- **Basic program structure**: Understanding the `main` function
- **Output**: Using the `println!` macro
- **Macros**: Introduction to Rust's macro system
- **Compilation**: How to compile and run Rust code

## The Hello World Program

In the file [`hello.rs`](./hello.rs), you'll find a simple "Hello, World!" program:

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

You should see `Hello, World!` printed to your terminal.

## Your First Challenge

1. Modify the program to print your name instead of "World"
2. Add a second line that prints "Welcome to Rust!"
3. Experiment with printing different types of values

## What's Next?

Now that you've created your first Rust program, you're ready to learn about variables and data types, which are covered in the next section.