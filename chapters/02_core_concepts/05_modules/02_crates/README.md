# Crates in Rust

A crate is a compilation unit in Rust. It's the smallest amount of code that the Rust compiler considers at a time.

## What is a Crate?

A crate can produce either:
- An executable (binary crate)
- A library (library crate)

Crates are the building blocks that make up packages. Every package contains one or more crates.

## Types of Crates

### Binary Crates

- Compile to an executable that you can run
- Must have a `main` function that defines what happens when the executable runs
- The entry point file is typically `src/main.rs`

### Library Crates

- Don't compile to an executable
- Intended to be used in other programs
- Can be shared on [crates.io](https://crates.io)
- The entry point file is typically `src/lib.rs`

## Crate Root

Each crate has a root file, which is the entry point for the compiler:

- For a binary crate, the default root is `src/main.rs`
- For a library crate, the default root is `src/lib.rs`

The crate root forms a module named `crate` at the root of the crate's module tree.

## Using External Crates

To use an external crate, you need to:

1. Add it to your `Cargo.toml` file:
   ```toml
   [dependencies]
   rand = "0.8.5"
   ```

2. Bring its items into scope in your code:
   ```rust
   use rand::Rng;
   
   fn main() {
       let random_number = rand::thread_rng().gen_range(1..=100);
       println!("Random number: {}", random_number);
   }
   ```

## Crate Features

Crates can define optional "features" that can be enabled or disabled:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

This allows a crate to provide optional functionality without requiring users to include code they don't need.

## Creating Your Own Crates

### Binary Crate Example

```rust
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```

### Library Crate Example

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(4, add(2, 2));
    }
}
```

## Practice Exercise

Open the [`2_crates.rs`](./2_crates.rs) file and complete the exercises to test your understanding of Rust crates.

## Next Steps

Now that you understand crates, we can move on to [modules basics](../03_modules_basics/README.md), which are the organizational units within crates. 