# Pattern Matching in Rust

Pattern matching is a powerful feature in Rust that allows you to destructure and check values against patterns. It's used extensively with enums, but can also match against literals, ranges, variables, and more complex structures.

## Key Concepts

- **Match Expressions**: The primary way to use pattern matching
- **Pattern Types**: Different kinds of patterns you can match against
- **Destructuring**: Breaking compound values into their components
- **Match Guards**: Adding extra conditions to patterns
- **Bindings**: Capturing values in patterns for later use

## Match Expressions

The `match` keyword in Rust starts a pattern matching expression:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    // More patterns...
    _ => default_expression,
}
```

Match expressions must be exhaustive - they must cover all possible values of the matched type.

## Pattern Types

Rust supports many types of patterns:

### Literals

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

### Named Variables

```rust
let x = Some(5);

match x {
    Some(y) => println!("Matched, y = {}", y),
    None => println!("No match"),
}
```

### Multiple Patterns

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    _ => println!("other"),
}
```

### Ranges

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("other"),
}
```

## Destructuring

Pattern matching can destructure various compound types:

### Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("At position ({}, {})", x, y),
}
```

### Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
}
```

### Nested Structures

```rust
let nested = (1, (2, 3));

match nested {
    (a, (b, c)) => println!("a: {}, b: {}, c: {}", a, b, c),
}
```

## Match Guards

Match guards add additional conditions to patterns:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("is: {}", x),
    None => println!("is None"),
}
```

## Bindings with @ Operator

The `@` operator lets you capture a value while also testing it against a pattern:

```rust
let num = Some(5);

match num {
    Some(n @ 1..=5) => println!("Got a number in range: {}", n),
    Some(n) => println!("Got a different number: {}", n),
    None => println!("Got None"),
}
```

## Ignoring Values

Rust provides several ways to ignore parts of patterns:

### Using _ for Entire Values

```rust
let some_value = 5;
match some_value {
    _ => println!("Don't care what the value is"),
}
```

### Using _ for Parts of Values

```rust
let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

## Shorthand Forms

Rust provides shorter forms for common pattern matching scenarios:

### If Let

When you only care about one pattern:

```rust
let some_value = Some(3);

if let Some(3) = some_value {
    println!("three");
}
```

### While Let

For conditional loops based on patterns:

```rust
let mut stack = Vec::new();
// ...push some values...

while let Some(top) = stack.pop() {
    println!("{}",  top);
}
```

## Practice Exercise

Open [`0_pattern_matching.rs`](./0_pattern_matching.rs) and complete the challenges related to pattern matching:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Match expressions
   - Destructuring complex types
   - Using match guards
   - Conditional binding with @
   - Ignoring values in patterns

Run the program to verify that your solutions pass all the tests.

## What's Next?

After mastering pattern matching, we'll explore modules, which allow you to organize your code and control its visibility. 