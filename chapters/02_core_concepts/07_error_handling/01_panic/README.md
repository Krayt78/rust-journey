# Panic in Rust

Panic is Rust's mechanism for handling **unrecoverable errors** - situations where the program can't proceed safely.

## When a Panic Occurs

When a panic is triggered:
1. The program starts unwinding the stack (cleaning up resources)
2. An error message is printed with information about where the panic occurred
3. The program exits

## When to Use Panic

Panic should be used in specific scenarios:

- **Unrecoverable situations**: When it's impossible to handle an error and continue safely
- **Contract violations**: When a function's contract is violated (e.g., out-of-bounds array access)
- **During prototyping**: Before implementing proper error handling
- **Impossible cases**: When you've checked all possible error cases and the remaining case should never occur
- **Tests**: To fail tests when unexpected behavior is encountered

## Sources of Panics

Panics can come from several sources:

1. **Explicit** `panic!` macro:
   ```rust
   panic!("This is a panic message");
   ```

2. **Standard library functions** that panic:
   ```rust
   let v = vec![1, 2, 3];
   let item = v[99]; // Panics with index out of bounds
   ```

3. **Assertion macros**:
   ```rust
   assert!(condition, "Panic message if condition is false");
   assert_eq!(a, b, "Panic message if a != b");
   assert_ne!(a, b, "Panic message if a == b");
   ```

4. **Unwrapping None or Err values**:
   ```rust
   let x: Option<i32> = None;
   let y = x.unwrap(); // Panics

   let result: Result<i32, &str> = Err("Error");
   let z = result.unwrap(); // Panics
   ```

## Handling Panics

You generally don't "handle" panics in the way you handle other errors, but you can:

1. **Use `Result` instead**: Convert potential panics to `Result` types when possible
2. **Set panic behavior**: Choose between stack unwinding (default) or aborting:
   ```toml
   # In Cargo.toml
   [profile.release]
   panic = "abort"  # Options: "unwind" or "abort"
   ```
3. **Catch panics**: Use `std::panic::catch_unwind()` (advanced, not recommended for normal error handling)

## Practice

Open the [0_panic.rs](./0_panic.rs) file to see examples of panics and try the exercises.

## Next Steps

After learning about panic, proceed to the [Result](../02_result/README.md) section to understand how to handle recoverable errors. 