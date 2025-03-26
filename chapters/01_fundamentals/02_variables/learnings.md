# Variables in Rust

This document explains how variables work in Rust, including immutability, mutability, shadowing, and constants.

For more information, see [The Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)


## Immutable Variables

By default, variables in Rust are immutable (cannot be changed after being set):

```rust
let language = "Rust";
println!("I'm learning {}", language);

// This would cause a compile error:
// language = "Rust Programming";
```

Immutability helps prevent bugs and makes code easier to reason about, especially in concurrent contexts.

## Mutable Variables

When you need a variable to change, you can use the `mut` keyword:

```rust
let mut version = "1.0";
println!("Starting with version: {}", version);

// Now we can change it
version = "1.72";
println!("Updated to version: {}", version);
```

## Shadowing

Rust allows you to declare a new variable with the same name as a previous variable, which "shadows" the original:

```rust
let count = 5;
println!("Original count: {}", count);

// This creates a new variable that shadows the previous one
let count = count + 5;
println!("After shadowing: {}", count);
```

Shadowing is different from mutation because we're actually creating a new variable. This allows us to change the type of the value but reuse the same name:

```rust
let value = "42";
println!("Value as string: {}", value);

let value = value.parse::<i32>().unwrap();
println!("Value as integer: {}", value);
```

## Constants

Constants are values that are bound to a name and cannot change:

```rust
const MAX_USERS: u32 = 100_000;
println!("Maximum users allowed: {}", MAX_USERS);

// Notice how we use underscores for readability in numbers
const PI: f64 = 3.141_592_653_589_793;
println!("Pi: {}", PI);
```

Note that:
- The type must be annotated
- Constants are always immutable (no `mut` keyword needed)
- Constants can be declared in any scope
- Constants must be set to a constant expression, not a value computed at runtime
- By convention, constants use SCREAMING_SNAKE_CASE naming

## Variable Scope

Variables have a scope, which is the block they're declared in:

```rust
{
    // This variable only exists in this block
    let scoped_variable = "I'm only visible inside this block";
    println!("Inside block: {}", scoped_variable);
}
// This would cause a compile error:
// println!("Outside block: {}", scoped_variable);
```

## Practice Challenges

For practice, try these challenges:

1. Fix a mutable variable declaration
2. Implement proper shadowing
3. Fix a constant declaration
4. Fix a complex shadowing example
5. Correct a boolean shadowing issue

You can find these challenges in the `challenge.rs` file, along with tests to verify your solutions. 