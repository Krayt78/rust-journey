# Unwrap and Expect in Rust

Rust provides shorthand methods `unwrap()` and `expect()` for quickly extracting values from `Result` and `Option` types in situations where you expect success or are willing to panic on failure.

## Understanding Unwrap

The `unwrap()` method:
- Extracts the value from `Ok` (for `Result`) or `Some` (for `Option`)
- Panics if the value is `Err` or `None`

```rust
// Using unwrap with Result
let file = File::open("config.txt").unwrap(); // Panics if file doesn't exist

// Using unwrap with Option
let first_item = vec![1, 2, 3].first().unwrap(); // Safe, vector is not empty
let first_item = vec![].first().unwrap(); // Panics, vector is empty
```

## Understanding Expect

The `expect()` method is similar to `unwrap()`, but allows you to specify a custom panic message:

```rust
// Using expect with Result
let file = File::open("config.txt").expect("Failed to open config file");

// Using expect with Option
let user = get_user(123).expect("User not found");
```

## When to Use Unwrap and Expect

These methods should be used judiciously:

### Appropriate Uses:

1. **Prototyping and examples**: When you're in the early stages of development
2. **Test code**: When a failure indicates a broken test
3. **When failure is impossible**: When you've already checked the error condition
4. **When a panic is the correct response**: When recovery is not possible

```rust
// Examples of reasonable unwrap usage:

// After checking the condition
if let Ok(value) = potentially_failing_operation() {
    let result = value.next_operation().unwrap(); // We know this can't fail
}

// In test code
#[test]
fn test_feature() {
    let result = process_data().unwrap(); // Test should fail if processing fails
    assert_eq!(result, expected);
}
```

### Inappropriate Uses:

1. **Production error handling**: Where graceful recovery is possible
2. **Public API functions**: Where callers should decide how to handle errors
3. **Unvalidated external input**: Where failures are expected and normal

## Safer Alternatives

Instead of `unwrap()` and `expect()`, consider:

1. **Pattern matching**:
   ```rust
   match result {
       Ok(value) => /* use value */,
       Err(error) => /* handle error */,
   }
   ```

2. **Providing fallbacks with `unwrap_or` and `unwrap_or_else`**:
   ```rust
   // Use a default value if Result is Err or Option is None
   let number = "not a number".parse::<i32>().unwrap_or(0);
   
   // Compute a fallback value with a closure
   let user = get_user(id).unwrap_or_else(|| User::default());
   ```

3. **Propagating errors with `?`**:
   ```rust
   fn read_config() -> Result<Config, io::Error> {
       let file = File::open("config.txt")?; // Returns error if file doesn't exist
       // Continue with file
   }
   ```

## Practice

Open the [0_unwrap.rs](./0_unwrap.rs) file to see examples of unwrap and expect and try the exercises.

## Next Steps

After learning about unwrap and expect, proceed to [Error Propagation](../04_propagation/README.md) to understand how to pass errors up the call stack. 