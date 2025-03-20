# Variables in Rust

Variables are a fundamental concept in any programming language. In Rust, variables have some unique characteristics that contribute to the language's safety guarantees.

## Key Concepts

- **Immutability**: Variables are immutable by default
- **Mutability**: Using the `mut` keyword to make variables mutable
- **Shadowing**: Creating a new variable with the same name
- **Constants**: Defining values that never change

## Variable Declarations

In [`variables.rs`](./variables.rs), you'll find examples of variable declarations and challenges:

```rust
fn main() {
    // Immutable variable (default)
    let name = "Ferris";
    
    // Mutable variable
    let mut age = 3;
    age = 4; // This is allowed because age is mutable
    
    // Shadowing
    let language = "Rust";
    let language = language.len(); // This creates a new variable with the same name
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
}
```

### Immutability

By default, variables in Rust are immutable - once a value is bound to a name, you cannot change that value:

```rust
let x = 5;
x = 6; // This will cause an error!
```

This design choice helps prevent bugs and makes code easier to reason about, especially in concurrent contexts.

### Mutability

When you need a variable to change, you can use the `mut` keyword:

```rust
let mut x = 5;
x = 6; // This works fine!
```

### Shadowing

Rust allows you to declare a new variable with the same name as a previous variable:

```rust
let x = 5;
let x = x + 1; // This creates a new variable, also named x
```

Shadowing is different from mutation because we're actually creating a new variable. This allows us to change the type of the value but reuse the same name.

### Constants

Constants are values that are bound to a name and cannot change:

```rust
const MAX_POINTS: u32 = 100_000;
```

Note that:
- The type must be annotated
- Constants are always immutable (no `mut` keyword needed)
- Constants can be declared in any scope
- Constants must be set to a constant expression, not a value computed at runtime

## Learning Through Challenges

Each file in this course has two parts:

1. **Teaching Section**: The top part of each file contains working examples that demonstrate concepts with explanations in comments.

2. **Challenges Module**: A module at the bottom of each file with intentionally broken code that you need to fix. These challenges test your understanding of the concepts presented in the teaching section.

To complete the challenges:
1. Read through the teaching section to understand the concepts
2. Look at the broken code in the challenges module
3. Fix each issue to make the code compile and run correctly
4. The code contains validations that will tell you if your fixes are correct

## Practice Exercise

Open [`variables.rs`](./variables.rs) and complete the challenges related to variables:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Variable mutability
   - Shadowing
   - Constants
   - Type changes through shadowing

Run the program to verify that your solutions pass all the tests.

## What's Next?

Now that you understand variables, let's explore the different data types available in Rust.