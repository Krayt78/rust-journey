// Generic Types in Rust
//
// This file demonstrates how to use generic types to write
// flexible, reusable code that works with multiple types.

fn main() {
    println!("Understanding Generic Types in Rust!");
    
    //------------------------------------------------------
    // GENERIC FUNCTIONS
    //------------------------------------------------------
    println!("\n=== Generic Functions ===");
    
    // A function that works with any printable type
    print_value(42);
    print_value("hello");
    print_value(true);
    
    // A function with multiple generic parameters
    let result = combine("Hello, ", "world!");
    println!("Combined: {}", result);
    
    //------------------------------------------------------
    // GENERIC STRUCTS
    //------------------------------------------------------
    println!("\n=== Generic Structs ===");
    
    // Point with the same type for x and y
    let int_point = Point { x: 5, y: 10 };
    println!("Integer point: x={}, y={}", int_point.x, int_point.y);
    
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Float point: x={}, y={}", float_point.x, float_point.y);
    
    // Point with different types for x and y
    let mixed_point = MixedPoint { x: 5, y: 4.0 };
    println!("Mixed point: x={}, y={}", mixed_point.x, mixed_point.y);
    
    //------------------------------------------------------
    // METHODS ON GENERIC TYPES
    //------------------------------------------------------
    println!("\n=== Methods on Generic Types ===");
    
    // Calling a method on a generic struct
    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());
    
    // Method that uses additional generic types
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: "World" };
    let p3 = p1.mixup(p2);
    println!("Mixed-up point: p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    //------------------------------------------------------
    // GENERIC ENUMS
    //------------------------------------------------------
    println!("\n=== Generic Enums ===");
    
    // Option<T> is a generic enum in the standard library
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // Check and unwrap Option values
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }
    
    // Result<T, E> is another generic enum
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    
    match success {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    //------------------------------------------------------
    // TYPE CONSTRAINTS
    //------------------------------------------------------
    println!("\n=== Type Constraints ===");
    
    // Generic function with trait bounds
    let numbers = vec![34, 50, 25, 100, 65];
    let largest = find_largest(&numbers);
    println!("The largest number is {}", largest);
    
    let characters = vec!['y', 'm', 'a', 'q'];
    let largest = find_largest(&characters);
    println!("The largest character is {}", largest);
    
    //------------------------------------------------------
    // PERFORMANCE OF GENERICS
    //------------------------------------------------------
    println!("\n=== Performance of Generics ===");
    
    println!("Rust uses monomorphization to generate specialized code");
    println!("for each concrete type used with generics.");
    println!("This means generic code has zero runtime cost!");
    
    // These two function calls generate different specialized code at compile time
    let int_result = identity(42);
    let str_result = identity("hello");
    println!("int_result: {}, str_result: {}", int_result, str_result);
}

// A generic function that works with any type that can be formatted with Debug
fn print_value<T: std::fmt::Debug>(value: T) {
    println!("Value: {:?}", value);
}

// A generic function with multiple type parameters
fn combine<T, U>(t: T, u: U) -> String 
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    format!("{}{}", t, u)
}

// A generic struct where x and y must be the same type
struct Point<T> {
    x: T,
    y: T,
}

// Generic method implementation
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic method with additional type parameter
impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> Point<T, U> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// A generic struct with different types for x and y
struct Point<T, U> {
    x: T,
    y: U,
}

// Renamed to avoid name conflict with the previous Point
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// A generic function with a type constraint (must implement PartialOrd)
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// A simple identity function to demonstrate monomorphization
fn identity<T>(t: T) -> T {
    t
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Generic Pair
//
// Implement a generic struct called `Pair<T>` that holds two values of the same type.
// Then implement a method called `new` that creates a new pair,
// and a method called `get_max` that returns the largest value.
// The `get_max` method should only work for types that can be compared.

// TODO: Implement the Pair struct and its methods
// struct Pair<T> { ... }
// impl<T> Pair<T> { ... }
// impl<T: PartialOrd> Pair<T> { ... }

// Exercise 2: Generic Data Storage
//
// Implement a generic data storage type `Storage<T>` that can store items of any type.
// The storage should have methods to add items, get an item by index, and return the count of items.

// TODO: Implement the Storage struct and its methods
// struct Storage<T> { ... }
// impl<T> Storage<T> { ... }

// Exercise 3: Converting Collections
//
// Implement a function `convert_collection` that can convert a Vec<T> into a different collection type
// (for this exercise, convert to a HashSet<T>).
// The function should work only for types that can be hashed and compared for equality.

use std::collections::HashSet;
use std::hash::Hash;

// TODO: Implement the convert_collection function
// fn convert_collection<T: ...>(vec: Vec<T>) -> HashSet<T> { ... }

// Exercise 4: Generic Binary Tree
//
// Implement a binary tree data structure that can store any type.
// Include methods to insert a value and check if a value exists in the tree.
// The tree should work for any type that can be compared for equality and ordering.

// TODO: Implement the BinaryTree type and its methods
// enum BinaryTree<T> { ... }
// impl<T: Ord> BinaryTree<T> { ... }

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Exercise 1
    #[test]
    fn test_pair_creation() {
        let pair = Pair::new(1, 5);
        assert_eq!(pair.first, 1);
        assert_eq!(pair.second, 5);
    }

    #[test]
    fn test_pair_get_max() {
        let pair1 = Pair::new(5, 10);
        assert_eq!(pair1.get_max(), 10);
        
        let pair2 = Pair::new("apple", "banana");
        assert_eq!(pair2.get_max(), "banana");
    }

    // Tests for Exercise 2
    #[test]
    fn test_storage() {
        let mut storage = Storage::new();
        storage.add(1);
        storage.add(2);
        storage.add(3);
        
        assert_eq!(storage.count(), 3);
        assert_eq!(storage.get(1), Some(&2));
        assert_eq!(storage.get(3), None);
    }

    // Tests for Exercise 3
    #[test]
    fn test_convert_collection() {
        let vec = vec![1, 2, 3, 3, 4];
        let set = convert_collection(vec);
        
        assert_eq!(set.len(), 4);
        assert!(set.contains(&1));
        assert!(set.contains(&4));
    }

    // Tests for Exercise 4
    #[test]
    fn test_binary_tree() {
        let mut tree = BinaryTree::Empty;
        tree = tree.insert(5);
        tree = tree.insert(3);
        tree = tree.insert(7);
        
        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&7));
        assert!(!tree.contains(&8));
    }
} 