# Integration Testing in Rust

Integration tests verify that multiple components of your code work together correctly. Unlike unit tests, which test individual functions or methods in isolation, integration tests focus on the interactions between different parts of your code.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 11.3: Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) in the official Rust Book.

## Integration Test Structure in Rust

In Rust, integration tests:
- Are located in the `tests` directory at the root of your project
- Each file in the `tests` directory is compiled as a separate crate
- Have access to the public API of your crate, just like external code would

This structure ensures your tests exercise your code as it would be used by other crates.

## Creating Integration Tests

To create integration tests for a project:

1. Create a `tests` directory at the same level as your `src` directory
2. Add one or more Rust files in the `tests` directory
3. Write tests that use your crate's public API

Here's an example of a simple integration test:

```rust
// In tests/integration_test.rs

// Import the crate we're testing
use my_crate;

#[test]
fn test_public_function() {
    // Call a public function from our crate
    assert!(my_crate::public_function());
}

#[test]
fn test_another_function() {
    // Test another public function
    assert_eq!(my_crate::calculate(2, 3), 5);
}
```

## Shared Test Utility Code

If you need to share code between multiple test files, you can create modules in the `tests` directory:

```
my_project/
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs  // Shared test code
    ├── basic_tests.rs
    └── advanced_tests.rs
```

You can then use this shared code in your test files:

```rust
// In tests/basic_tests.rs

// Import the common module
mod common;

#[test]
fn test_with_common_setup() {
    // Use functions from the common module
    let data = common::setup();
    assert!(my_crate::process(&data));
}
```

## Testing Binary Crates

If your project is a binary crate (with a `main.rs` file), you might want to separate your code into a library crate and a thin binary wrapper to make it easier to test.

```rust
// In src/lib.rs
pub fn logic_function() -> bool {
    true
}

// In src/main.rs
use my_crate::logic_function;

fn main() {
    if logic_function() {
        println!("It works!");
    }
}
```

Then you can test the library's functionality:

```rust
// In tests/integration_test.rs
use my_crate;

#[test]
fn test_logic_function() {
    assert!(my_crate::logic_function());
}
```

## When to Use Integration Tests

Integration tests are valuable when:

- You want to test the public API of your crate
- You need to verify that different modules work together correctly
- You're testing functionality that spans multiple components
- You want to ensure your crate is usable from an external perspective

## Integration vs. Unit Tests

Both types of tests are important:

| Unit Tests | Integration Tests |
|------------|-------------------|
| Test individual components | Test how components work together |
| Can test private functions | Can only test public API |
| In the same file as the code | In a separate `tests` directory |
| Faster to run | May be slower to run |
| Help with design decisions | Validate overall functionality |

## Practice

Open the Rust file in this directory to explore the examples and complete the exercises:
- [0_integration_testing.rs](./0_integration_testing.rs) - Examples and exercises for integration testing

## Key Points

- Integration tests verify that different parts of your code work together correctly
- They are located in the `tests` directory at the root of your project
- Each test file is compiled as a separate crate with access to your crate's public API
- Integration tests complement unit tests by focusing on the public API and cross-component interactions

## Next Steps

After mastering integration testing, proceed to the next folder to learn about documentation testing, which allows you to ensure your code examples in documentation are correct and up-to-date. 