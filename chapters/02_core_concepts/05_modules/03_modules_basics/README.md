# Module Basics in Rust

Modules are the organizational units within Rust's module system. They allow you to group related code together and control the privacy of items.

## What are Modules?

Modules in Rust are used to:
- Organize code into logical groups
- Control the privacy of items (public vs. private)
- Create namespace boundaries

## Defining Modules

Modules are defined using the `mod` keyword:

```rust
// Define a module
mod garden {
    // Code inside the module
    pub fn grow_plant() {
        println!("The plant is growing!");
    }
}
```

## Module Hierarchies

Modules can be nested to create a hierarchy:

```rust
mod garden {
    pub mod vegetables {
        pub fn plant_veggie() {
            println!("Planting vegetables!");
        }
    }
    
    pub mod fruits {
        pub fn plant_fruit() {
            println!("Planting fruits!");
        }
    }
}
```

This creates a tree structure:

```
crate
 └── garden
     ├── vegetables
     │   └── plant_veggie
     └── fruits
         └── plant_fruit
```

## In-line Modules vs. File Modules

Modules can be defined in two ways:

1. **In-line modules**: Defined directly in a file with curly braces
   ```rust
   // In main.rs or lib.rs
   mod garden {
       // Module contents here
   }
   ```

2. **File modules**: Defined in separate files
   ```rust
   // In main.rs or lib.rs
   mod garden; // Load from garden.rs or garden/mod.rs
   ```

## Module Files

When using file modules, you have two options:

1. **Same-name file**:
   - `mod garden;` in `lib.rs` will load from `garden.rs`

2. **Directory with mod.rs**:
   - `mod garden;` in `lib.rs` will load from `garden/mod.rs`

For submodules:
- `mod vegetables;` in `garden.rs` will load from `garden/vegetables.rs`
- Or from `garden/vegetables/mod.rs`

## 2018 Edition Module System

In Rust 2018 edition and later, you can also use:

```
src/garden.rs          # declares the garden module
src/garden/vegetables.rs  # declares the garden::vegetables submodule
```

Without needing a `mod.rs` file, making the structure clearer.

## Module Functions and Items

Modules can contain functions, structs, enums, and other items:

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    
    pub fn calculate_area(shape: &str) -> f64 {
        // Implementation
        0.0
    }
}
```

## Practice Exercise

Open the [`3_modules_basics.rs`](./3_modules_basics.rs) file and complete the exercises to test your understanding of Rust modules.

## Next Steps

Now that you understand the basics of modules, we can move on to [paths](../04_paths/README.md), which are how we reference items in the module hierarchy. 