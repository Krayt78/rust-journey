# Enums in Rust

Enums (short for enumerations) allow you to define a type by enumerating its possible variants. They're especially useful when you need to represent data that could be one of several different types or configurations.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) in the official Rust Book.

## Key Concepts

- **Basic Enums**: Creating types with different variants
- **Enums with Data**: Variants that contain data of different types
- **Option Enum**: Rust's built-in solution for nullable values
- **Match Expressions**: Pattern matching with enums
- **Methods on Enums**: Defining behavior for enum types

## Defining and Using Enums

A basic enum is defined with the `enum` keyword followed by a name and a list of variants:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

To create an instance of an enum, you specify one of its variants:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## Enums with Data

Enum variants can store associated data, and different variants can have different types:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

You can also define more complex data structures:

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields like a struct
    Write(String),              // String data
    ChangeColor(i32, i32, i32), // Three i32 values
}
```

## The Option Enum

Rust doesn't have null, but it has the `Option` enum which expresses that a value could be something or nothing:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option` is so useful that it's included in the prelude; you don't need to bring it into scope explicitly:

```rust
let some_number = Some(5);     // Some variant with i32 value
let some_string = Some("text"); // Some variant with &str value
let absent_number: Option<i32> = None; // Need type annotation with None
```

## Match Expressions with Enums

Match is a powerful control flow operator for enums:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Pattern Matching with Option

The `Option` enum is commonly used with match expressions:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Methods on Enums

Like structs, enums can have methods:

```rust
enum Message {
    Write(String),
    // other variants...
}

impl Message {
    fn call(&self) {
        // Method body would use self to do something with the variant
        match self {
            Message::Write(text) => println!("Writing: {}", text),
            // Handle other variants...
        }
    }
}

let msg = Message::Write(String::from("hello"));
msg.call();
```

## Practice Exercise

Open [`0_enums.rs`](./0_enums.rs) and complete the challenges related to enums:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Creating and using enums
   - Enums with different data types
   - Working with the Option enum
   - Pattern matching on enums
   - Implementing methods for enums

Run the program to verify that your solutions pass all the tests.

## What's Next?

After mastering enums, we'll explore pattern matching in more depth, which provides powerful ways to destructure and work with complex data types. 