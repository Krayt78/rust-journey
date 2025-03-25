# Unit Testing in Rust

Unit tests verify that individual pieces of your code work correctly in isolation. They are essential for ensuring that functions, methods, and other components behave as expected.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 11.1: How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) in the official Rust Book.

## Location of Unit Tests

In Rust, unit tests typically:
- Are located in the same file as the code they test
- Are placed in a module annotated with `#[cfg(test)]`
- Use functions annotated with `#[test]`

This approach keeps the tests close to the implementation, making it easier to understand and maintain both.

## Writing Basic Unit Tests

Here's a simple example of unit testing in Rust:

```rust
// The function we want to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// The test module
#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;
    
    #[test]
    fn test_add() {
        // Test the add function
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
```

The `#[cfg(test)]` attribute tells Rust to compile and run this code only when running tests. This means your test code doesn't affect the size or performance of your compiled program.

## Test-Driven Development (TDD)

Test-driven development is a practice where you:
1. Write a failing test for the feature you want to implement
2. Implement the feature just enough to make the test pass
3. Refactor your code while keeping the tests passing

This cycle is often referred to as "Red-Green-Refactor":
- Red: Write a failing test
- Green: Make the test pass
- Refactor: Improve the code without changing its behavior

TDD can lead to more robust code and better design decisions.

## Testing for Failures

Sometimes, you need to test that your code fails in expected ways. Rust provides the `#[should_panic]` attribute for this:

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
}
```

The `expected` parameter is optional but recommended as it verifies that the test panics for the reason you expect.

## Test Organization

As your tests grow, you might want to organize them into multiple functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_positive_numbers() {
        // Tests with positive numbers
    }
    
    #[test]
    fn test_negative_numbers() {
        // Tests with negative numbers
    }
    
    #[test]
    fn test_edge_cases() {
        // Tests for edge cases
    }
}
```

This organization makes your tests more readable and maintainable.

## Testing Private Functions

In Rust, you can test private functions directly since the test module is part of the same crate:

```rust
fn private_function(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private_function() {
        assert_eq!(private_function(3), 6);
    }
}
```

This allows thorough testing without exposing implementation details.

## Practice

Open the Rust file in this directory to explore the examples and complete the exercises:
- [0_unit_testing.rs](./0_unit_testing.rs) - Examples and exercises for unit testing

## Key Points

- Unit tests help verify that individual components work correctly
- In Rust, unit tests are in the same file as the code they test
- The `#[cfg(test)]` attribute ensures test code is only compiled during testing
- Test-driven development can lead to more robust code
- Tests should cover both success and failure cases

## Next Steps

After mastering unit testing, proceed to the next folder to learn about integration testing, which verifies that multiple components work together correctly. 