# Documentation Testing in Rust

Documentation tests in Rust ensure that your code examples in documentation work correctly. They serve a dual purpose: they provide accurate usage examples for users of your code, and they act as tests to verify that your code works as documented.

## What Are Documentation Tests?

In Rust, code examples in documentation comments (those marked with `///` or `//!`) are automatically run as tests when you run `cargo test`. This ensures that:

1. Your examples are syntactically correct
2. Your examples actually work
3. Your examples stay up-to-date with your code

## Writing Documentation Tests

Documentation tests are written as part of your code's documentation. Here's an example:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

When you run `cargo test`, this code snippet in the documentation is extracted, compiled, and run as a test.

## Documentation Test Syntax

Documentation tests use Markdown syntax. A code block is introduced with three backticks (\`\`\`), optionally followed by the language name:

```rust
/// ```
/// // This is a basic code example
/// ```
///
/// ```rust
/// // This is a Rust code example (with syntax highlighting in documentation)
/// ```
```

## Hiding Parts of the Example

Sometimes, you need to include code in your example that shouldn't be visible in the documentation. You can do this with the `# ` prefix:

```rust
/// ```
/// # use my_crate::User;
/// let user = User::new("Alice");
/// assert_eq!(user.name(), "Alice");
/// ```
```

The line with the `# ` prefix will be executed during testing but won't appear in the generated documentation.

## Indicating Expected Errors

If your example is expected to panic or not compile, you can add annotations:

```rust
/// ```should_panic
/// // This code panics
/// my_crate::function_that_panics();
/// ```
///
/// ```compile_fail
/// // This code won't compile
/// let x: i32 = "not an integer";
/// ```
```

## Ignoring Documentation Tests

If you need to include a code example that shouldn't be run as a test, you can mark it:

```rust
/// ```ignore
/// // This code won't be tested
/// let x = unfinished_function();
/// ```
```

## Testing Private Functions

Since documentation tests are treated as if they're external to your crate, they can only access public items. If you need to document private functions, you can use plain comments (`//`) or include examples that won't be tested.

## Module Documentation

You can document entire modules using the `//!` syntax at the top of the file:

```rust
//! # My Module
//!
//! This module provides utilities for working with users.
//!
//! # Examples
//!
//! ```
//! use my_crate::user_module::create_user;
//! let user = create_user("Alice");
//! assert_eq!(user.name(), "Alice");
//! ```

// Rest of the module code...
```

## Benefits of Documentation Tests

Documentation tests provide several benefits:

1. **Verified examples**: Your documentation examples are guaranteed to work
2. **Always up-to-date**: If your API changes, tests will fail until examples are updated
3. **Better documentation**: Forces you to think about clear, concise examples
4. **Added test coverage**: More tests without writing separate test files

## Practice

Open the Rust file in this directory to explore the examples and complete the exercises:
- [0_doc_testing.rs](./0_doc_testing.rs) - Examples and exercises for documentation testing

## Key Points

- Documentation tests ensure that examples in your documentation are correct and up-to-date
- They are written directly in documentation comments using Markdown code blocks
- Special annotations can indicate expected failures or parts to hide from documentation
- Doc tests run as if they were external code, so they can only access public items
- Documentation tests encourage better API design and more comprehensive examples

## Next Steps

After mastering documentation testing, proceed to the next folder to learn about test organization strategies, including how to structure tests for larger projects, use fixtures, and implement test-driven development practices. 