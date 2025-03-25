# Paths in Rust

In Rust's module system, paths are how you find an item in the module tree. They allow you to navigate and access items (functions, structs, enums, etc.) within modules.

## Types of Paths

Rust supports two types of paths:

1. **Absolute paths**: Start from the crate root by using the crate name or the `crate` literal
2. **Relative paths**: Start from the current module and use `self`, `super`, or an identifier in the current module

## Absolute Paths

Absolute paths start from the crate root:

```rust
// Starting with the crate name (for external crates)
std::collections::HashMap

// Starting with 'crate' (for the current crate)
crate::garden::vegetables::Carrot
```

## Relative Paths

Relative paths start from the current module:

```rust
// Referring to an item in the current module
self::plant_seeds()

// Referring to an item in the parent module
super::garden::plant_seeds()

// Referring to an item in a child module
vegetables::plant_carrot()
```

## The `self` Keyword

The `self` keyword refers to the current module:

```rust
mod plant {
    pub fn grow() {
        self::water(); // Refers to the water function in the same module
    }
    
    fn water() {
        println!("Watering the plant");
    }
}
```

## The `super` Keyword

The `super` keyword refers to the parent module:

```rust
mod garden {
    fn tend_garden() {
        println!("Tending garden");
    }
    
    mod vegetables {
        pub fn plant() {
            // Call a function in the parent module
            super::tend_garden();
        }
    }
}
```

## Multiple `super` References

You can use multiple `super` keywords to navigate up multiple levels:

```rust
mod level_1 {
    fn function_1() {}
    
    mod level_2 {
        mod level_3 {
            fn function_3() {
                // Go up two levels
                super::super::function_1();
            }
        }
    }
}
```

## Ambiguous References

When there are multiple items with the same name in different modules, you need to be explicit about which one you're referring to:

```rust
mod plants {
    pub fn grow() {
        println!("Growing plants");
    }
}

mod animals {
    pub fn grow() {
        println!("Growing animals");
    }
}

fn main() {
    // Ambiguous - which grow() function?
    // grow();
    
    // Be explicit
    plants::grow();
    animals::grow();
}
```

## Practice Exercise

Open the [`4_paths.rs`](./4_paths.rs) file and complete the exercises to test your understanding of paths in Rust's module system.

## Next Steps

Now that you understand paths, we can move on to [privacy and visibility](../05_privacy_and_visibility/README.md), which control what items can be accessed from which modules. 