# Generic Types, Traits, and Lifetimes

This chapter covers three of Rust's most powerful features for writing flexible and reusable code:

- **Generic Types** allow you to define code that works with multiple types
- **Traits** provide shared behavior across different types
- **Lifetimes** ensure references remain valid for as long as they're needed

## Corresponding Section in the Rust Book

This section corresponds to [Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) in the official Rust Book.

## Generic Types

Generics are a way to write code that can work with multiple types. Instead of duplicating code for each specific type, you can write a single implementation that works for any type matching your requirements.

### Generic Functions

You can define functions that work with any type using generic type parameters:

```rust
fn print_value<T>(value: T) {
    println!("Value: {:?}", value);
}
```

The `<T>` syntax declares a generic type parameter, which can represent any type that implements the `Debug` trait (needed for `{:?}` formatting).

### Generic Structs

Structs can also use generic type parameters:

```rust
struct Point<T> {
    x: T,
    y: T,
}

// Using different concrete types
let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

In this example, both `x` and `y` must be the same type, but we can create `Point` instances with different concrete types.

### Generic Structs with Multiple Type Parameters

You can use multiple generic type parameters:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

// x and y can now have different types
let mixed_point = Point { x: 5, y: 4.0 };
```

### Generic Enums

Enums can also be generic:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

These are standard library enums that you've likely already seen.

### Generic Method Implementations

You can implement methods on generic types:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method implementations can have additional type parameters
impl<T, U> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> Point<T, U> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

### Performance of Generics

Rust implements generics using a technique called monomorphization. At compile time, Rust generates specialized code for each concrete type used. This means:

- Generic code has zero runtime cost compared to concrete implementations
- The compiled code is as efficient as if you had written separate functions for each type
- The binary size may increase due to multiple specialized versions of the same code

## Traits

Traits define shared behavior across different types. They are similar to interfaces in other languages but with some unique features.

### Defining Traits

A trait defines a set of methods that a type must implement:

```rust
trait Summary {
    fn summarize(&self) -> String;
    
    // Traits can have default implementations
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### Implementing Traits

You implement a trait for a specific type:

```rust
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // We can choose to use the default implementation
    // or override it with our own implementation
}
```

### Trait Bounds

You can restrict generic types to those that implement specific traits:

```rust
// T must implement the Display and PartialOrd traits
fn largest<T: PartialOrd + Display>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
```

### Where Clauses

For complex trait bounds, you can use `where` clauses for clearer code:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // implementation
}
```

### Trait Objects

Trait objects allow for dynamic dispatch:

```rust
// A vector that can hold different types that implement Summary
let articles: Vec<Box<dyn Summary>> = vec![
    Box::new(NewsArticle { /* ... */ }),
    Box::new(Tweet { /* ... */ }),
];

// We can call methods on the trait object
for article in articles {
    println!("{}", article.summarize());
}
```

### Marker Traits

Some traits don't define any methods but mark that a type has certain properties:

```rust
// These are built-in marker traits
trait Send {}      // Type can be transferred across thread boundaries
trait Sync {}      // Type can be referenced from multiple threads
trait Copy {}      // Type can be copied with a bitwise copy (memcpy)
```

### Associated Types

Traits can define associated types for more flexible APIs:

```rust
trait Iterator {
    type Item;  // An associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Lifetimes

Lifetimes ensure that references are valid for as long as they need to be. They prevent dangling references by making the borrow checker's rules explicit in function signatures and types.

### Lifetime Annotations

Lifetime parameters are named with an apostrophe (`'`) prefix:

```rust
// 'a is a lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In this example, the lifetime parameter `'a` indicates that both parameters and the return value must live at least as long as the lifetime `'a`.

### Lifetime Annotations in Structs

Structs can also have lifetime parameters:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    // excerpt cannot outlive novel because it references data from novel
}
```

### Lifetime Elision Rules

Rust has a set of rules that allow you to omit lifetime annotations in common cases:

1. Each parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters

### Static Lifetime

The `'static` lifetime indicates that a reference can live for the entire duration of the program:

```rust
// String literals have the 'static lifetime
let s: &'static str = "I have a static lifetime.";
```

### Combining Generic Types, Trait Bounds, and Lifetimes

You can combine all these features:

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Best Practices

1. **Start concrete, then generalize**: Begin with specific types and refactor to generics as needed
2. **Use trait bounds to restrict types**: Only allow types that support the operations you need
3. **Prefer associated types for cleaner API design**: When a trait should only have one implementation for a type
4. **Be explicit with lifetimes when necessary**: Sometimes the compiler needs help understanding the relationships between references

## Practice

This chapter is organized into several sections for easier learning:

- [01_generics](./01_generics/) - Learn about generic types and how to use them
- [02_traits](./02_traits/) - Explore traits and shared behavior across types
- [03_lifetimes](./03_lifetimes/) - Understand lifetime annotations and borrow checking
- [04_combined_example](./04_combined_example/) - See how to combine all three concepts

Each section contains its own README with more specific information and a Rust file with examples and exercises.

## Next Steps

After understanding generics, traits, and lifetimes, you'll be ready to explore Rust's functional programming features such as closures and iterators. 