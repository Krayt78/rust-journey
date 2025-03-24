# Borrowing in Rust

Borrowing is a fundamental concept in Rust that allows you to reference data without taking ownership of it. This solves many of the challenges presented by the ownership system while still maintaining memory safety.

## Key Concepts

- **References**: How to create references to values without taking ownership
- **Borrowing Rules**: The rules that govern how references can be used
- **Mutable References**: How to modify borrowed values
- **Immutable References**: Using references for read-only access
- **Dangling References**: How Rust prevents references to invalid data

## References and Borrowing

References allow you to refer to a value without taking ownership of it:

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // Pass a reference to s1
println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
```

The `&` symbol indicates a reference, and the action of creating a reference is called "borrowing".

## Borrowing Rules

The borrowing system follows these rules:

1. At any given time, you can have either:
   - One mutable reference
   - Any number of immutable references

2. References must always be valid (Rust prevents dangling references)

## Mutable References

To modify borrowed data, you need a mutable reference:

```rust
let mut s = String::from("hello");
change(&mut s); // Pass a mutable reference

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Immutable References

You can have multiple immutable references at the same time:

```rust
let s = String::from("hello");
let r1 = &s; // First immutable reference
let r2 = &s; // Second immutable reference
println!("{} and {}", r1, r2); // Both references are valid
```

But you cannot have a mutable reference and an immutable reference at the same time:

```rust
let mut s = String::from("hello");
let r1 = &s; // Immutable reference
let r2 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
```

## Preventing Dangling References

Rust prevents dangling references - references to data that has been deallocated:

```rust
fn dangle() -> &String { // Error: would return a reference to a dropped value
    let s = String::from("hello");
    &s
} // s goes out of scope and is dropped, so the reference would be invalid
```

## Practice Exercise

Open [`0_borrowing.rs`](./0_borrowing.rs) and complete the challenges related to borrowing:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Creating and using references
   - Mutable vs. immutable borrowing
   - Avoiding dangling references
   - The borrow checker rules

Run the program to verify that your solutions pass all the tests.

## What's Next?

After mastering ownership and borrowing, you'll learn about structs, which allow you to create custom data types by grouping related values together. 