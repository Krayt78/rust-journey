# Control Flow in Rust

Control flow in programming refers to the order in which statements are executed. Rust provides common control flow constructs like if statements, loops, and match expressions to direct the flow of your program.

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 3.5: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) in the official Rust Book.

## Key Concepts

- **If Expressions**: Conditional branching with return values
- **Loops**: Three types - `loop`, `while`, and `for`
- **Match Expressions**: Pattern matching with exhaustive checking
- **If let / While let**: Simplified pattern matching

## If Expressions

Rust's `if` expressions allow you to branch your code based on conditions. Unlike many languages, `if` is an expression in Rust, which means it returns a value.

```rust
let number = 5;

if number < 0 {
    println!("The number is negative");
} else if number == 0 {
    println!("The number is zero");
} else {
    println!("The number is positive");
}
```

You can also use `if` in a `let` statement:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

## Loops

Rust provides three kinds of loops: `loop`, `while`, and `for`.

### Loop

The `loop` keyword creates an infinite loop that runs until explicitly told to stop:

```rust
loop {
    println!("This will print forever unless we break");
    
    // Use break to exit the loop
    break;
}
```

You can also return values from loops:

```rust
let result = loop {
    // Do something
    break 42; // Return 42 from the loop
};
```

### While

`while` loops run as long as a condition is true:

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");
```

### For

`for` loops are used to iterate over elements of a collection:

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("The value is: {}", element);
}
```

You can also use `for` with a range:

```rust
for number in 1..4 {
    println!("{}!", number);
}
```

It's important to note that ranges in Rust are exclusive of the upper bound by default. The range `1..4` includes 1, 2, and 3, but not 4. If you want to include the upper bound, you can use the inclusive range syntax with `..=`:

```rust
// This will print 1, 2, 3, 4
for number in 1..=4 {
    println!("{}!", number);
}
```

## Match Expressions

`match` is a powerful control flow operator in Rust that allows you to compare a value against a series of patterns and execute code based on which pattern matches:

```rust
let number = 1;

match number {
    1 => println!("One!"),
    2 => println!("Two!"),
    3 => println!("Three!"),
    _ => println!("Other number"),
}
```

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

Open [`0_control_flow.rs`](./0_control_flow.rs) and complete the challenges related to control flow:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - If expressions
   - Loops (loop, while, for)
   - Match expressions
   - If let syntax

Run the program to verify that your solutions pass all the tests.

## What's Next?

Now that you understand control flow in Rust, you're ready to explore Rust's collection types in the next section. 