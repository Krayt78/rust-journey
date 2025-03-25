# Ownership in Rust

Ownership is one of Rust's most unique and important features. It enables Rust to make memory safety guarantees without needing a garbage collector.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 4.1: What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) in the official Rust Book.

## Key Concepts

- **Ownership Rules**: The foundational rules that govern how memory is managed
- **Move Semantics**: How values are transferred between variables
- **Copy Types**: Types that are copied rather than moved
- **Functions and Ownership**: How passing values to functions affects ownership
- **Return Values and Scope**: How returning values transfers ownership

## Ownership Rules

Three fundamental rules govern Rust's ownership system:

1. Each value in Rust has a variable that is its "owner"
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

## Move Semantics

When you assign a variable to another variable, Rust "moves" the value from one owner to another:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer valid
```

After this move, you cannot use `s1` anymore. This prevents double-free errors and other memory issues.

## Copy Types

Simple scalar types (like integers, floats, booleans) are "copied" rather than moved:

```rust
let x = 5;
let y = x; // x is still valid because integers are Copy types
```

Types that implement the `Copy` trait are copied, not moved.

## Functions and Ownership

When you pass a value to a function, ownership is transferred to the function:

```rust
fn take_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped

let s = String::from("hello");
take_ownership(s); // s's value moves into the function
// s is no longer valid here
```

## Return Values and Scope

Functions can also transfer ownership back to the caller:

```rust
fn give_ownership() -> String {
    let s = String::from("hello");
    s // s is returned and ownership moves to the calling function
}

let s = give_ownership(); // s takes ownership of the returned value
```

## Practice Exercise

Open [`0_ownership.rs`](./0_ownership.rs) and complete the challenges related to ownership:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Understanding move semantics
   - Copy types
   - Function ownership transfers
   - Returning ownership

Run the program to verify that your solutions pass all the tests.

## What's Next?

After understanding ownership, the next logical step is to learn about borrowing, which allows you to reference data without taking ownership. 