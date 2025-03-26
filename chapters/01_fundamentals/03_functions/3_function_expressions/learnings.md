# Function Expressions in Rust

This document explains the concept of expressions in Rust functions and how they affect return values.

For more information, see [The Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Statements vs. Expressions

Rust makes an important distinction between statements and expressions:

- **Statements** are instructions that perform an action but don't return a value
- **Expressions** evaluate to a resulting value

In Rust, most things are expressions, which is different from many other programming languages.

Examples of statements:
- Variable declarations with `let`
- Function declarations
- Semicolon-terminated lines

Examples of expressions:
- Literal values like `5` or `"hello"`
- Mathematical operations like `5 + 6`
- Function calls like `add(5, 7)`
- Blocks surrounded by `{}`

## Expressions as Return Values

In Rust, functions return the value of their final expression. This means you don't always need to use the `return` keyword:

```rust
fn add(a: i32, b: i32) -> i32 {
    // This is an expression (no semicolon)
    a + b
}
```

The expression `a + b` is evaluated and its value is automatically returned.

Adding a semicolon would turn this expression into a statement, which would not return a value:

```rust
// This would cause an error because it doesn't return a value
fn broken_add(a: i32, b: i32) -> i32 {
    a + b;  // Semicolon makes this a statement that returns ()
}
```

## Conditional Expressions

In Rust, `if` is an expression, which means it returns a value:

```rust
fn check_number(x: i32) -> &'static str {
    // if-else is an expression in Rust that returns a value
    if x > 10 {
        "greater than ten"
    } else {
        "less than or equal to ten"
    }
}
```

The entire `if-else` block is an expression that returns one of the two string literals.

## Block Expressions

Rust allows you to create blocks of code with curly braces `{}`. These blocks are also expressions:

```rust
fn compute_value(x: i32) -> i32 {
    // This entire block is an expression
    {
        let a = x * 2;
        let b = a + 10;
        // The last expression in the block is returned
        b - 5
    }
}
```

A block can contain multiple statements, and its value is the value of the final expression.

## Early Returns vs. Expression-Based Returns

You can use the `return` keyword for early returns in a function:

```rust
fn absolute_value_with_return(n: i32) -> i32 {
    if n < 0 {
        return -n;  // Early return for negative numbers
    }
    
    n  // Return n if it's positive or zero
}
```

However, it's often more idiomatic in Rust to use expression-based returns:

```rust
fn absolute_value_expression(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}
```

Both functions do the same thing, but the second one uses Rust's expression-based style.

## Common Mistakes with Expressions

When working with expressions in functions, common mistakes include:

1. Adding semicolons to expressions that should return values
   ```rust
   // Error: Doesn't return a value because of the semicolon
   fn grade_message(score: i32) -> &'static str {
       if score >= 90 {
           "Excellent";  // Semicolon makes this a statement
       } else {
           "Good";  // Semicolon makes this a statement
       }
   }
   
   // Correct: No semicolons on return expressions
   fn grade_message(score: i32) -> &'static str {
       if score >= 90 {
           "Excellent"
       } else {
           "Good"
       }
   }
   ```

2. Using `return` when it's not necessary
   ```rust
   // Unnecessarily verbose
   fn absolute_value(n: i32) -> i32 {
       if n < 0 {
           return -n;
       } else {
           return n;
       }
   }
   
   // More idiomatic Rust
   fn absolute_value(n: i32) -> i32 {
       if n < 0 {
           -n
       } else {
           n
       }
   }
   ```

3. Missing expressions at the end of a block
   ```rust
   // Error: No expression at the end of the block
   fn factorial(n: u32) -> u32 {
       {
           let mut result = 1;
           
           for i in 1..=n {
               result *= i;
           }
           
           result;  // Semicolon makes this a statement, not the return value
       }
   }
   
   // Correct: Expression at the end of the block
   fn factorial(n: u32) -> u32 {
       {
           let mut result = 1;
           
           for i in 1..=n {
               result *= i;
           }
           
           result  // No semicolon - this value is returned
       }
   }
   ```

## Practice Challenges

To practice working with function expressions, try fixing these common issues:

1. Converting a function that uses `return` statements to use expression-based returns
2. Fixing a function that has semicolons after expressions that should be returned
3. Correcting a block expression to properly return a value

You can find these challenges in the `challenge.rs` file. 