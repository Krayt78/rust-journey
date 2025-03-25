# Privacy and Visibility in Rust

One of the key features of Rust's module system is the ability to control the privacy of items. Privacy rules determine what code can access which items.

## Default Privacy

In Rust, all items (functions, methods, structs, enums, modules, and constants) are **private by default**. This means they can only be accessed by code in the same module or its child modules.

## The `pub` Keyword

The `pub` keyword makes an item public, allowing it to be accessed from outside its defining module:

```rust
mod garden {
    pub fn grow_plant() {  // Public function
        water_soil();      // Can call private functions
    }
    
    fn water_soil() {      // Private function
        println!("Watering the soil");
    }
}

fn main() {
    garden::grow_plant();  // This works because grow_plant is public
    // garden::water_soil(); // ERROR - water_soil is private
}
```

## Privacy Rules

1. If an item is public, it can be accessed by outside code that has visibility to its parent module
2. If an item is private, it can only be accessed by code in the same module or by child modules

## Structs and Privacy

Structs have an additional level of privacy: each field can have its own visibility:

```rust
mod garden {
    pub struct Plant {
        pub name: String,      // Public field
        pub species: String,   // Public field
        id: u32,               // Private field
    }
    
    impl Plant {
        pub fn new(name: String, species: String) -> Plant {
            Plant {
                name,
                species,
                id: 42,        // We can set private fields inside this module
            }
        }
    }
}

fn main() {
    let mut plant = garden::Plant::new(
        String::from("Venus Flytrap"),
        String::from("Dionaea muscipula"),
    );
    
    plant.name = String::from("Modified Plant");  // OK - name is public
    // plant.id = 99;  // ERROR - id is private
}
```

## Enums and Privacy

When an enum is made public, all of its variants are also public:

```rust
mod garden {
    pub enum PlantType {
        Flowering,     // All variants are public
        NonFlowering,  // because the enum is public
        Fern,
        Moss,
    }
}

fn main() {
    let plant_type = garden::PlantType::Flowering;  // This is allowed
}
```

## Modules and Privacy

To use a nested module from outside its parent, both the parent module and the nested module must be public:

```rust
mod garden {
    pub mod vegetables {  // vegetables is public
        pub fn plant() {  // plant is public
            println!("Planting vegetables!");
        }
    }
    
    mod fruits {          // fruits is private
        pub fn plant() {  // plant is public, but not accessible from outside
            println!("Planting fruits!");
        }
    }
}

fn main() {
    garden::vegetables::plant();  // This works
    // garden::fruits::plant();   // ERROR - fruits is private
}
```

## Impl Blocks and Privacy

Items defined in `impl` blocks follow the same privacy rules:

```rust
mod garden {
    pub struct Plant {
        pub name: String,
    }
    
    impl Plant {
        pub fn new(name: String) -> Plant {  // Public constructor
            Plant { name }
        }
        
        fn private_helper(&self) {           // Private method
            println!("Helper function for internal use");
        }
    }
}
```

## Privacy and Use Statements

The `use` keyword doesn't change privacy rules. You can only bring public items into scope:

```rust
mod garden {
    pub mod vegetables {
        pub fn plant() {
            println!("Planting vegetables!");
        }
    }
}

use garden::vegetables::plant;  // OK - plant is public

fn main() {
    plant();  // We can call it directly now
}
```

## Practice Exercise

Open the [`5_privacy_and_visibility.rs`](./5_privacy_and_visibility.rs) file and complete the exercises to test your understanding of Rust's privacy and visibility rules.

## Next Steps

Now that you understand privacy and visibility, we can move on to [the use keyword](../06_use_keyword/README.md), which helps bring items into scope for easier access. 