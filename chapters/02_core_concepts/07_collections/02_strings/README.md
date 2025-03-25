# Strings in Rust

Rust strings can be confusing for newcomers because the language has two main string types: `String` and `&str`. This section will clarify the differences and show you how to work with strings effectively.

## String Types

### `String`

`String` is a growable, mutable, owned string type that is allocated on the heap:

- **Owned**: The `String` owns its data and is responsible for freeing memory when it goes out of scope
- **Growable**: Can be modified to add or remove content
- **Heap-allocated**: The content is stored on the heap

### `&str` (String Slice)

`&str` is an immutable view into a string, represented as a pointer to UTF-8 encoded text with a length:

- **Borrowed**: It's a reference to UTF-8 text owned by someone else
- **Immutable**: Cannot be modified directly
- **Fixed-size**: The size is known at compile time or determined at runtime but cannot change

## Creating Strings

```rust
// Create a new, empty String
let mut s = String::new();

// Create a String from a string literal
let s = String::from("hello");
let s = "hello".to_string();

// Create a String with capacity
let mut s = String::with_capacity(10);
```

## String Literals vs. Strings

String literals (`&str`) are stored in the program's binary and are immutable:

```rust
let s = "Hello, world!"; // String literal (&str)
```

`String` is a mutable, growable UTF-8 encoded text stored on the heap:

```rust
let s = String::from("Hello, world!"); // String
```

## Updating Strings

### Adding to Strings

```rust
let mut s = String::from("Hello");

// Append a string slice
s.push_str(", world");

// Append a single character
s.push('!');

println!("{}", s); // "Hello, world!"
```

### String Concatenation

```rust
// Using + operator (consumes the first String)
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 is moved here and can no longer be used

// Using format! macro (doesn't consume any arguments)
let s1 = String::from("Hello");
let s2 = String::from("world");
let s3 = format!("{}, {}!", s1, s2);
```

## Indexing and Slicing Strings

Rust doesn't support direct indexing of strings using the `[]` operator because a `String` is UTF-8 encoded and characters may be multiple bytes.

```rust
let s = String::from("Hello");
// let h = s[0]; // This will not compile!

// Instead, you can use slices
let hello = &s[0..5];
```

Caution: Slicing a string must occur at valid UTF-8 character boundaries or the program will panic.

## Iterating Over Strings

```rust
let s = String::from("नमस्ते"); // Namaste in Hindi

// Iterate over bytes
for b in s.bytes() {
    println!("{}", b);
}

// Iterate over Unicode scalar values
for c in s.chars() {
    println!("{}", c);
}
```

## String Methods

| Method            | Description                              | Example                             |
|-------------------|------------------------------------------|-------------------------------------|
| `len()`           | Returns the length in bytes              | `s.len()`                           |
| `is_empty()`      | Checks if the string is empty            | `s.is_empty()`                      |
| `push_str()`      | Appends a string slice                   | `s.push_str("world")`               |
| `push()`          | Appends a character                      | `s.push('!')`                       |
| `replace()`       | Replaces occurrences of a pattern        | `s.replace("Hello", "Hi")`          |
| `trim()`          | Removes whitespace from start and end    | `s.trim()`                          |
| `split()`         | Splits string by a delimiter             | `s.split(" ")`                      |
| `contains()`      | Checks if string contains a pattern      | `s.contains("world")`               |
| `to_uppercase()`  | Converts to uppercase                    | `s.to_uppercase()`                  |
| `to_lowercase()`  | Converts to lowercase                    | `s.to_lowercase()`                  |
| `starts_with()`   | Checks if string starts with pattern     | `s.starts_with("Hello")`            |
| `ends_with()`     | Checks if string ends with pattern       | `s.ends_with("world!")`             |

## UTF-8 and String Representation

Rust strings are UTF-8 encoded, which means:

1. A character may take between 1 and 4 bytes
2. Not all byte sequences are valid characters
3. The length of a string in bytes may not equal the number of characters
4. Indexing must be done at character boundaries

For example:
```rust
let hello = String::from("Hello");
let namaste = String::from("नमस्ते");

println!("Length of 'Hello': {} bytes", hello.len()); // 5 bytes
println!("Length of 'नमस्ते': {} bytes", namaste.len()); // 18 bytes (6 characters)
println!("'नमस्ते' has {} characters", namaste.chars().count()); // 6 characters
```

## Practice Exercise

Open the [`2_strings.rs`](./2_strings.rs) file and complete the exercises to test your understanding of strings in Rust.

## Next Steps

Now that you understand strings, you can proceed to [Hash Maps](../03_hash_maps/README.md), which allow you to associate keys with values. 