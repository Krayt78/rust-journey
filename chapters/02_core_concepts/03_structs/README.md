# Structs in Rust

Structs (short for structures) are custom data types that let you group related values together and name them to make your code more clear.

## Key Concepts

- **Basic Structs**: Creating custom types with named fields
- **Tuple Structs**: Struct variants with unnamed fields
- **Unit Structs**: Structs without any fields
- **Methods**: Functions attached to structs
- **Associated Functions**: Functions associated with a struct but that don't operate on an instance

## Defining and Instantiating Structs

A basic struct is defined with the `struct` keyword followed by a name and a list of fields:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To create an instance of a struct, you specify concrete values for each field:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

## Accessing and Modifying Struct Fields

You can access struct fields using dot notation:

```rust
let email = user1.email;
```

If the struct instance is mutable, you can change field values:

```rust
let mut user1 = User { /* fields */ };
user1.email = String::from("newemail@example.com");
```

## Field Init Shorthand

When variables and fields have the same name, you can use the field init shorthand:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // Instead of email: email
        username, // Instead of username: username
        active: true,
        sign_in_count: 1,
    }
}
```

## Struct Update Syntax

You can create a new struct instance from an existing one using update syntax:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // All other fields come from user1
};
```

## Tuple Structs

Tuple structs are similar to regular structs but their fields don't have names:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## Methods and Associated Functions

Methods are functions attached to structs. They always take `self` as their first parameter:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method that takes an instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function (doesn't take self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());

// Call an associated function with ::
let square = Rectangle::square(20);
```

## Practice Exercise

Open [`0_structs.rs`](./0_structs.rs) and complete the challenges related to structs:

1. Fix the code to make it compile and run correctly
2. Solve the various challenges related to:
   - Creating and using structs
   - Implementing methods
   - Using associated functions
   - Tuple structs

Run the program to verify that your solutions pass all the tests.

## What's Next?

After mastering structs, we'll explore enums, which allow you to define a type that can be one of several variants. 