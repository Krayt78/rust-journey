# Function Parameters in Rust

This document explains how to define and use function parameters in Rust.

For more information, see [The Rust Book: Function Parameters](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#parameters)

## Function Parameter Basics

In Rust, function parameters are defined in the function signature by providing a name and a type for each parameter:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

In this example:
- `name` is the parameter name
- `&str` is the parameter type (string slice)

## Calling Functions with Arguments

When calling a function with parameters, you provide arguments that match the expected types:

```rust
// Calling a function with a single parameter
greet("Rustacean");

// Calling with a different argument
greet("World");
```

## Multiple Parameters

Functions can have multiple parameters, separated by commas:

```rust
fn describe_person(name: &str, age: i32) {
    println!("{} is {} years old", name, age);
}

// Calling a function with multiple parameters
describe_person("Alice", 30);
```

## Expressions as Arguments

Arguments can be expressions that evaluate to the required type:

```rust
let x = 10;
let y = 20;
add_and_print(x + 5, y);
```

## Type Annotations

In Rust, all function parameters must have type annotations. This enforces Rust's strong typing system and helps with error checking and code safety:

```rust
// Good: Type is specified
fn multiply(a: i32, b: i32) {
    println!("{} * {} = {}", a, b, a * b);
}

// Would not compile: Missing type annotation for b
// fn multiply(a: i32, b) { ... }
```

## Optional Parameters with Option

Rust doesn't have default parameter values like some languages, but you can emulate this behavior using the `Option` type:

```rust
fn greet_with_optional_title(name: &str, title: Option<&str>) {
    match title {
        Some(t) => println!("Hello, {} {}!", t, name),
        None => println!("Hello, {}!", name),
    }
}

// Using with no title
greet_with_optional_title("Ada", None);

// Using with a title
greet_with_optional_title("Grace", Some("Dr."));
```

## Parameter Order

The order of parameters matters when calling a function. Arguments are matched to parameters in the same order they are defined:

```rust
// The function expects (from, to) in that order
fn send_greeting(from: &str, to: &str) {
    println!("From {}: Hello, {}!", from, to);
}

// Must call in the correct order
send_greeting("Bob", "Charlie");  // "From Bob: Hello, Charlie!"
```

If the parameters are in the wrong order or the function body uses them incorrectly, the output will be wrong:

```rust
// Incorrect implementation: parameters used in wrong order
fn send_greeting_wrong(from: &str, to: &str) {
    println!("From {}: Hello, {}!", to, from);  // Wrong order in the print statement
}

send_greeting_wrong("Bob", "Charlie");  // Would print "From Charlie: Hello, Bob!"
```

## Common Mistakes with Parameters

When working with function parameters, common mistakes include:

1. Forgetting to specify a parameter type
   ```rust
   // Error: Missing type for parameter b
   fn multiply(a: i32, b) {
       println!("{} * {} = {}", a, b, a * b);
   }
   ```

2. Missing required parameters
   ```rust
   // Error: Function needs name and age parameters
   fn describe_person() {
       println!("{} is {} years old", name, age);
   }
   ```

3. Using parameters in the wrong order
   ```rust
   // Error: Parameters used in wrong order in body
   fn send_greeting(from: &str, to: &str) {
       println!("From {}: Hello, {}!", to, from);
   }
   ```

## Practice Challenges

To practice working with function parameters, try fixing these common issues:

1. Adding a missing parameter type
2. Adding missing parameters to a function signature
3. Correcting parameter usage within a function body

You can find these exact challenges in the `challenge.rs` file. 