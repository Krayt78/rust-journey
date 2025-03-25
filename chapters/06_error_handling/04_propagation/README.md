# Error Propagation in Rust

Error propagation is the practice of passing errors up the call stack to let calling code decide how to handle them. Rust provides elegant mechanisms to propagate errors without excessive boilerplate.

## The Traditional Way

Without specialized syntax, error propagation looks like this:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Early return with the error
    };
    
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // Return the error
    }
}
```

## The ? Operator

The `?` operator simplifies error propagation:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Return error if Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // Return error if Err
    Ok(username)
}
```

When you apply `?` to a `Result`, it:
1. Returns the error early if the result is `Err`
2. Extracts the value if the result is `Ok`

## The ? Operator with Option

The `?` operator also works with `Option`:

```rust
fn first_char(text: &str) -> Option<char> {
    let text = text.trim();
    (!text.is_empty()).then_some(())?; // Return None if empty
    text.chars().next()
}
```

When you apply `?` to an `Option`, it:
1. Returns `None` early if the option is `None`
2. Extracts the value if the option is `Some`

## Chaining Operations

The `?` operator enables concise chaining of operations that might fail:

```rust
fn read_and_process() -> Result<ProcessedData, Error> {
    let data = read_file("input.txt")?;
    let parsed = parse_data(data)?;
    let processed = process_data(parsed)?;
    Ok(processed)
}
```

## Error Type Conversion

The `?` operator automatically converts error types if the `From` trait is implemented:

```rust
fn read_and_process() -> Result<ProcessedData, CustomError> {
    let file = File::open("input.txt")?; // io::Error converted to CustomError
    let data = parse_file(file)?; // ParseError converted to CustomError
    // Process data...
    Ok(ProcessedData { /* ... */ })
}

// Allows automatic conversion from io::Error to CustomError
impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::Io(error)
    }
}

// Allows automatic conversion from ParseError to CustomError
impl From<ParseError> for CustomError {
    fn from(error: ParseError) -> Self {
        CustomError::Parse(error)
    }
}
```

## Where Can ? Be Used

The `?` operator can only be used in functions that return:
- `Result<T, E>` (for propagating `Result` errors)
- `Option<T>` (for propagating `Option` absence)
- Types that implement `Try` (advanced usage)

You can't use `?` in the `main()` function unless it returns a `Result`.

```rust
// This works:
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("file.txt")?;
    // Rest of the function...
    Ok(())
}
```

## Best Practices

1. **Be consistent**: Choose error propagation or handling at appropriate levels
2. **Add context**: Consider using `.map_err()` before `?` to add context to the error
3. **Document errors**: Make it clear which errors your function might return
4. **Consider error granularity**: Use specific error types where it adds value

## Practice

Open the [0_propagation.rs](./0_propagation.rs) file to see examples of error propagation and try the exercises.

## Next Steps

After learning about error propagation, proceed to [Option](../05_option/README.md) to understand how to handle potentially absent values. 