# Result in Rust

The `Result<T, E>` type is Rust's primary mechanism for handling **recoverable errors** - situations where an operation might fail, but the program can reasonably continue.

## Result Type Definition

The `Result` enum is defined as:

```rust
enum Result<T, E> {
    Ok(T),    // Success case containing a value of type T
    Err(E),   // Error case containing an error of type E
}
```

Where:
- `T` is the type of the success value
- `E` is the type of the error value

## Using Result

Result is used extensively in Rust's standard library and in many third-party libraries:

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("config.txt");
    
    match file_result {
        Ok(file) => {
            println!("File opened successfully!");
            // Use the file...
        },
        Err(error) => {
            println!("Error opening file: {:?}", error);
            // Handle the error...
        }
    }
}
```

## Pattern Matching with Result

You can use pattern matching to handle different error types:

```rust
use std::fs::File;
use std::io::ErrorKind;

match File::open("config.txt") {
    Ok(file) => {
        // Use the file
    },
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            // Create the file if it doesn't exist
            match File::create("config.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
        },
        other_error => {
            // Handle other errors
            panic!("Problem opening the file: {:?}", other_error);
        }
    },
}
```

## Shorthand Methods

Result provides several methods to simplify error handling:

### Unwrapping

```rust
// Unwrap yields the value or panics on error
let file = File::open("config.txt").unwrap();

// Expect does the same but with a custom panic message
let file = File::open("config.txt").expect("Failed to open config file");
```

### Error Propagation

Returning errors to the caller:

```rust
fn read_file() -> Result<String, io::Error> {
    let file = File::open("config.txt")?; // Short for "unwrap or return the error"
    // Continue processing...
}
```

### Result Combinators

Functional programming methods for working with Result:

```rust
// Transform the Ok value
let num_plus_one = "42".parse::<i32>().map(|num| num + 1);

// Transform the error
let file = File::open("file.txt").map_err(|e| format!("Failed to open: {}", e));

// Chain operations
let contents = File::open("file.txt").and_then(|mut file| {
    let mut contents = String::new();
    file.read_to_string(&mut contents).map(|_| contents)
});

// Provide a default value
let number = "not a number".parse::<i32>().unwrap_or(0);
```

## Best Practices

When using Result:

1. **Be explicit about error types** - use specific types that describe what went wrong
2. **Propagate errors where appropriate** - let callers decide how to handle errors
3. **Use unwrap/expect only** when a failure is truly unexpected or during prototyping
4. **Consider context** - add context to errors to make them more meaningful

## Practice

Open the [0_result.rs](./0_result.rs) file to see examples of Result and try the exercises.

## Next Steps

After learning about Result, proceed to [Unwrap & Expect](../03_unwrap/README.md) to learn more about shorthand techniques for handling Results. 