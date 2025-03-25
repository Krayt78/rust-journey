# Error Handling in Rust

Error handling is a critical aspect of writing robust, reliable software. Rust provides a rich set of tools for handling errors in a way that is both safe and expressive.

## Why Error Handling Matters

In any non-trivial program, things can go wrong:
- Files might not exist or can't be accessed
- Network connections can fail
- User input might be invalid
- System resources can be exhausted

Instead of crashing or producing incorrect results, well-designed software should:
1. Detect error conditions
2. Handle errors appropriately
3. Communicate errors clearly to users or calling code

Rust's approach to error handling encourages you to think about and handle potential failures explicitly, making your code more robust.

## Rust's Error Handling Philosophy

Rust takes a unique approach to error handling that differs from many other languages:

- **No Exceptions**: Unlike languages like Java, Python, or C++, Rust doesn't use exceptions for error handling.
- **Types for Error Handling**: Rust uses types (`Result<T, E>` and `Option<T>`) to represent operations that can fail.
- **Compiler Enforcement**: The compiler forces you to handle potential errors, making it difficult to accidentally ignore them.
- **Explicit Error Propagation**: Errors must be explicitly handled or propagated up the call stack.

## Error Handling Mechanisms

Rust provides several mechanisms for handling errors:

### 1. `panic!` and Unrecoverable Errors

The `panic!` macro is used for unrecoverable errors - situations where the program can't proceed safely:

```rust
fn main() {
    panic!("Critical error occurred!");
}
```

When a panic occurs:
- The program starts unwinding the stack (cleaning up resources)
- An error message is printed
- The program exits

Use panics when:
- It's impossible to recover from an error
- You're prototyping and don't want to handle errors yet
- The correct operation of the program can't be guaranteed (e.g., array access out of bounds)

### 2. `Result<T, E>` for Recoverable Errors

The `Result` enum is used for operations that can fail in a way that allows recovery:

```rust
enum Result<T, E> {
    Ok(T),    // Success case containing a value of type T
    Err(E),   // Error case containing an error of type E
}
```

Example of using `Result`:

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

### 3. `Option<T>` for Absent Values

The `Option` enum represents values that might be absent:

```rust
enum Option<T> {
    Some(T),  // Value is present
    None,     // Value is absent
}
```

Example of using `Option`:

```rust
fn find_user(id: u32) -> Option<User> {
    if id == 0 {
        None
    } else {
        Some(User { id, name: "Alice".to_string() })
    }
}

fn main() {
    match find_user(1) {
        Some(user) => println!("Found user: {}", user.name),
        None => println!("User not found")
    }
}
```

## Error Handling Patterns

### Pattern 1: Propagating Errors with `?`

The `?` operator provides a concise way to propagate errors:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // Returns early if Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Returns early if Err
    Ok(contents)
}
```

The `?` operator:
- Extracts the value from `Ok` or returns early with the `Err`
- Works with both `Result` and `Option`
- Automatically converts error types (when the `From` trait is implemented)

### Pattern 2: Combinators (`map`, `and_then`, etc.)

Combinators provide a functional approach to handling errors:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    File::open("username.txt")
        .map_err(|e| {
            println!("Error opening file: {:?}", e);
            e
        })
        .and_then(|mut file| {
            let mut username = String::new();
            file.read_to_string(&mut username)
                .map(|_| username)
        })
}
```

Common combinators:
- `map`: Transform the success value
- `map_err`: Transform the error value
- `and_then`: Chain operations that return `Result`
- `or_else`: Handle errors and return a new `Result`
- `unwrap_or`: Provide a default value in case of error
- `unwrap_or_else`: Compute a default value in case of error

### Pattern 3: Custom Error Types

For complex applications, creating custom error types can improve error handling:

```rust
#[derive(Debug)]
enum ApplicationError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    ConfigError(String),
}

impl From<std::io::Error> for ApplicationError {
    fn from(error: std::io::Error) -> Self {
        ApplicationError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for ApplicationError {
    fn from(error: std::num::ParseIntError) -> Self {
        ApplicationError::ParseError(error)
    }
}
```

Benefits of custom error types:
- Group related errors together
- Provide specific error handling based on error types
- Improve API documentation and usability
- Allow for better error messages

### Pattern 4: The `unwrap` and `expect` Methods

For quick prototyping or cases where errors are impossible, `unwrap` and `expect` can be used:

```rust
let file = File::open("config.txt").unwrap(); // Panics on error
let file = File::open("config.txt").expect("Failed to open config file"); // Panics with custom message
```

Use these methods with caution, as they will panic if an error occurs.

## Error Handling Libraries

Rust's ecosystem provides helpful libraries for error handling:

### thiserror

The `thiserror` crate makes creating custom error types easy:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Data parsing failed: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("Missing field: {0}")]
    MissingField(String),
}
```

### anyhow

The `anyhow` crate provides a simple `Result` type for applications where detailed error types aren't needed:

```rust
use anyhow::{Result, Context, bail, ensure};

fn process_data(path: &str) -> Result<()> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file {}", path))?;
    
    ensure!(!data.is_empty(), "File is empty");
    
    if data.contains("ERROR") {
        bail!("Data contains error marker");
    }
    
    // Process the data...
    Ok(())
}
```

## Best Practices for Error Handling

1. **Be explicit about what can fail**: Document the error conditions in functions that return `Result` or `Option`.

2. **Choose the right error type for the situation**:
   - Use `Result` for operations that can fail
   - Use `Option` for values that might be absent
   - Use `panic!` for unrecoverable errors

3. **Provide meaningful error messages**: Error messages should help users understand what went wrong and how to fix it.

4. **Consider the consumer of your API**: Library code should return detailed errors for the caller to handle, while applications might handle errors more directly.

5. **Don't overuse `unwrap` and `expect`**: These should be used only when you're certain the operation won't fail or during prototyping.

6. **Use type conversion to simplify error handling**: Implement the `From` trait to enable the `?` operator to convert between error types.

## Practice Exercise

Open the [`0_error_handling.rs`](./0_error_handling.rs) file and complete the exercises to test your understanding of error handling in Rust.

## Next Steps

After mastering error handling, you'll be ready to explore the functional features of Rust, including closures and iterators, which will enable you to write more expressive and concise code. 