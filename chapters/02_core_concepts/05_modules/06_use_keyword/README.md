# The `use` Keyword in Rust

The `use` keyword in Rust allows you to bring paths into scope, making it more convenient to reference items without typing out their full paths repeatedly.

## Basic Usage

The most basic usage of `use` is to bring an item into the current scope:

```rust
// Without use:
std::collections::HashMap::new();

// With use:
use std::collections::HashMap;
HashMap::new();
```

## Idiomatic Use Patterns

Rust has conventions for how to use the `use` keyword:

### For Functions

When importing functions, it's idiomatic to bring the parent module into scope, not the function itself:

```rust
// Preferred:
use std::fs;
fs::read_to_string("file.txt");

// Not idiomatic:
use std::fs::read_to_string;
read_to_string("file.txt");
```

This makes it clear where the function is coming from.

### For Structs, Enums, and Other Items

For other items, it's idiomatic to import the full path:

```rust
use std::collections::HashMap;
use std::io::Error;
```

## Nested Paths

You can combine multiple `use` statements that share a common prefix:

```rust
// Instead of:
use std::io;
use std::io::Write;

// You can write:
use std::io::{self, Write};
```

## Multiple Items from the Same Module

You can import multiple items from the same module in a single statement:

```rust
use std::collections::{HashMap, HashSet, BTreeMap};
```

## Renaming with `as`

The `as` keyword allows you to rename imports to avoid naming conflicts:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

## Re-exporting with `pub use`

The `pub use` syntax re-exports an item, making it available to external code as if it were defined in the current module:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Re-export
pub use crate::front_of_house::hosting;

// Now external code can use:
// my_crate::hosting::add_to_waitlist()
// Instead of:
// my_crate::front_of_house::hosting::add_to_waitlist()
```

This is useful for creating a clear public API.

## External Crates

To use an external crate, you need to:

1. Add it to your `Cargo.toml` file
2. Use the `use` keyword to bring its items into scope

```rust
// In Cargo.toml:
// [dependencies]
// rand = "0.8.5"

// In your code:
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
}
```

## The Glob Operator

The glob operator (`*`) brings all public items in a path into scope:

```rust
use std::collections::*;
```

This is generally avoided except in certain situations like testing, as it makes it unclear which items are in scope.

## Scoped Import

The `use` statement's scope is limited to the block it's declared in:

```rust
fn main() {
    // Only available in main
    use std::collections::HashMap;
    let mut map = HashMap::new();
}

fn other_function() {
    // Error: HashMap is not in scope here
    // let map = HashMap::new();
}
```

## Practice Exercise

Open the [`6_use_keyword.rs`](./6_use_keyword.rs) file and complete the exercises to test your understanding of the `use` keyword in Rust.

## Next Steps

Congratulations! You've completed the module on managing growing projects in Rust. Next, you're ready to explore more advanced concepts like error handling, generic types, traits, and lifetimes. 