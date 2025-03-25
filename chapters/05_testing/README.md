# Testing in Rust

Testing is a crucial part of software development that helps ensure your code works as expected and continues to work as you make changes. Rust has built-in support for testing through its test framework, making it easy to write and run tests.

## Why Testing Matters

- **Verifies correct behavior**: Tests confirm that your code does what it's supposed to do.
- **Prevents regressions**: Tests catch bugs that might be introduced when you modify your code.
- **Documents your code**: Tests serve as executable documentation, showing how your code is meant to be used.
- **Improves design**: Test-driven development (TDD) can lead to more modular, loosely coupled code.

## Types of Tests in Rust

Rust supports several types of tests, each with its own purpose:

### Unit Tests

Unit tests verify that individual components (functions, methods, etc.) work correctly in isolation. In Rust, unit tests typically:
- Live in the same file as the code they test
- Are placed in a `tests` module annotated with `#[cfg(test)]`
- Test small, isolated pieces of functionality

### Integration Tests

Integration tests verify that multiple components work together correctly. In Rust, integration tests:
- Live in the `tests` directory at the root of your project
- Each file in the `tests` directory is compiled as a separate crate
- Test how multiple parts of your library interact

### Documentation Tests

Documentation tests ensure that code examples in your documentation work correctly. In Rust:
- Code blocks in documentation comments (///) can be executed as tests
- They verify that your documentation examples are accurate and up-to-date
- They serve as both documentation and tests

## Test Structure in Rust

A typical test in Rust follows this pattern:

```rust
#[test]
fn test_something() {
    // 1. Setup - prepare any data or state
    let input = ...;
    
    // 2. Exercise - call the code you're testing
    let result = code_under_test(input);
    
    // 3. Verify - check that the result is what you expect
    assert_eq!(result, expected_output);
    
    // 4. Teardown (usually handled automatically in Rust)
}
```

## Common Testing Macros

Rust provides several assertion macros for use in tests:

- `assert!(expression)`: Ensures that an expression evaluates to `true`
- `assert_eq!(left, right)`: Ensures that two expressions are equal
- `assert_ne!(left, right)`: Ensures that two expressions are not equal
- `panic!("message")`: Causes the current thread to panic with the given message

## Test Attributes

You can modify test behavior with attributes:

- `#[test]`: Marks a function as a test
- `#[should_panic]`: Indicates that a test should cause a panic
- `#[should_panic(expected = "message")]`: Specifies the expected panic message
- `#[ignore]`: Temporarily skips a test when running tests

## Running Tests

Run your tests with the `cargo test` command:

```
$ cargo test                # Run all tests
$ cargo test test_name      # Run tests with names containing "test_name"
$ cargo test -- --nocapture # Show println! output from tests
$ cargo test -- --test-threads=1 # Run tests sequentially
```

## What You'll Learn in This Chapter

In this chapter, you'll learn how to:

1. Write effective unit tests for your Rust code
2. Create integration tests that verify components work together
3. Add documentation tests to ensure your examples work
4. Organize your tests for better maintainability
5. Use test-driven development (TDD) practices in Rust

## Sections in This Chapter

- [01_unit_testing](./01_unit_testing/) - Learn to write and run unit tests
- [02_integration_testing](./02_integration_testing/) - Create tests that verify components work together
- [03_doc_testing](./03_doc_testing/) - Write tests in your documentation
- [04_test_organization](./04_test_organization/) - Organize tests for maintainability

## Next Steps

After mastering testing in Rust, you'll be equipped to write reliable code with confidence. Testing is an essential skill that will serve you well as you tackle more complex Rust projects. 