# Function Return Values in Rust

This document explains how functions can return values in Rust.

For more information, see [The Rust Book: Functions - Return Values](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values)

## Basic Return Values

In Rust, functions can return values by specifying the return type after an arrow `->` in the function signature:

```rust
fn add(a: i32, b: i32) -> i32 {
    // The last expression in a function is returned
    // Note: no semicolon at the end, making it an expression, not a statement
    a + b
}

// Using the returned value
let result = add(5, 7);
println!("5 + 7 = {}", result);
```

## Implicit Returns

In Rust, the last expression in a function body is implicitly returned. This is different from many other languages that use a `return` keyword:

```rust
// This function returns the sum of two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = this is an expression that gets returned
}

// This would NOT work - adding a semicolon makes it a statement, not an expression
// fn broken_add(a: i32, b: i32) -> i32 {
//     a + b;  // With semicolon, this doesn't return a value
// }
```

## The Unit Type

Functions that don't explicitly return a value in Rust return the unit type `()`, which represents "no value":

```rust
// Function that doesn't explicitly return a value
fn say_hello() {
    println!("Hello!");
}

// The return value is (), the unit type
let nothing = say_hello();
println!("Value of nothing: {:?}", nothing);  // Prints: Value of nothing: ()
```

## Using Return Values in Expressions

Return values can be used directly in expressions without assigning them to variables:

```rust
// Using returned values directly in expressions
println!("15 + 27 = {}", add(15, 27));

// Doubling the result
let doubled = add(5, 7) * 2;
println!("Doubled: {}", doubled);
```

## Returning Multiple Values with Tuples

Rust doesn't directly support multiple return values, but you can return a tuple containing multiple values:

```rust
// Function that returns multiple values using a tuple
fn multiply_and_subtract(a: i32, b: i32) -> (i32, i32) {
    let product = a * b;
    let difference = a - b;
    // Return a tuple containing both values
    (product, difference)
}

// Using destructuring to get the individual values
let (product, difference) = multiply_and_subtract(10, 4);
println!("10 * 4 = {}, 10 - 4 = {}", product, difference);
```

## Early Returns

Rust allows early returns from functions using the `return` keyword. This is useful for conditional logic:

```rust
fn is_positive_or_zero(num: i32) -> bool {
    if num < 0 {
        return false;  // Early return for negative numbers
    }
    
    // If we get here, the number must be >= 0
    true
}

// Testing the function
let num = 42;
let result = is_positive_or_zero(num);
println!("{} is positive or zero: {}", num, result);  // true

let negative = -10;
let result = is_positive_or_zero(negative);
println!("{} is positive or zero: {}", negative, result);  // false
```

## Common Return Value Mistakes

When working with function return values, common mistakes include:

1. Adding a semicolon to the last expression, making it a statement instead of a return value
   ```rust
   // Error: Doesn't return a value
   fn square(num: i32) -> i32 {
       num * num;  // Semicolon makes this a statement, not a return value
   }
   
   // Correct
   fn square(num: i32) -> i32 {
       num * num  // No semicolon - this is now a return value
   }
   ```

2. Forgetting to return a value from all code paths
   ```rust
   // Error: Missing return in some branches
   fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
       if a > b {
           a  // Returns a if a > b
       } else {
           b  // Returns b if b >= a
       }
       // But what if c is the largest?
   }
   
   // Correct
   fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
       let max_ab = if a > b { a } else { b };
       if c > max_ab { c } else { max_ab }
   }
   ```

3. Incorrect syntax when returning multiple values
   ```rust
   // Error: Incorrect tuple syntax
   fn calculate(a: i32, b: i32) -> (i32, i32) {
       let sum = a + b;
       let product = a * b;
       sum, product  // This syntax is invalid
   }
   
   // Correct
   fn calculate(a: i32, b: i32) -> (i32, i32) {
       let sum = a + b;
       let product = a * b;
       (sum, product)  // Parentheses create a tuple
   }
   ```

## Practice Challenges

To practice working with function return values, try fixing these common issues:

1. Adding a function that returns the square of a number
2. Creating a function that returns a boolean indicating if a number is even
3. Implementing a function that returns multiple values as a tuple
4. Fixing a function to properly handle all possible return paths

You can find these challenges in the `challenge.rs` file. 