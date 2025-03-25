# Vectors in Rust

A vector (`Vec<T>`) is a resizable array type that can store elements of the same type. Vectors store their data on the heap, which means they can grow or shrink at runtime.

## Creating Vectors

There are several ways to create a vector:

### Using `Vec::new()`

```rust
let v: Vec<i32> = Vec::new(); // Empty vector of i32
```

### Using the `vec!` macro

```rust
let v = vec![1, 2, 3, 4, 5]; // Vector with initial values
```

### With a specific capacity

```rust
let v: Vec<i32> = Vec::with_capacity(10); // Space for 10 elements
```

## Updating Vectors

### Adding Elements

```rust
let mut v = Vec::new();
v.push(1);      // Add element to the end
v.push(2);
v.push(3);

// Insert at a specific position (shifts elements)
v.insert(1, 10); // Inserts 10 at index 1
```

### Removing Elements

```rust
let mut v = vec![1, 2, 3, 4, 5];
let last = v.pop();        // Removes and returns the last element
let second = v.remove(1);  // Removes and returns element at index 1
```

## Accessing Elements

### By Index

```rust
let v = vec![1, 2, 3, 4, 5];
let third = v[2]; // Zero-based indexing (gets 3)

// With bounds checking
match v.get(2) {
    Some(value) => println!("The third element is {}", value),
    None => println!("There is no third element"),
}
```

### Iterating Over Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

// Immutable references
for i in &v {
    println!("{}", i);
}

// Mutable references
let mut v = vec![1, 2, 3, 4, 5];
for i in &mut v {
    *i += 50; // Dereference to modify the value
}
```

## Vector Ownership

Like all data structures in Rust, vectors follow the ownership rules:

```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // Ownership moved to the function
    takes_ownership(v);
    
    // Error: v has been moved
    // println!("{:?}", v);
}

fn takes_ownership(v: Vec<i32>) {
    println!("Vector: {:?}", v);
    // v is dropped here
}
```

## Common Vector Methods

| Method           | Description                              | Example                          |
|------------------|------------------------------------------|----------------------------------|
| `len()`          | Returns the number of elements           | `v.len()`                        |
| `capacity()`     | Returns the allocated capacity           | `v.capacity()`                   |
| `is_empty()`     | Checks if the vector has no elements     | `v.is_empty()`                   |
| `clear()`        | Removes all elements                     | `v.clear()`                      |
| `extend()`       | Adds elements from an iterator           | `v.extend(other_vec)`            |
| `append()`       | Moves all elements from another vector   | `v.append(&mut other_vec)`       |
| `contains()`     | Checks if an element is in the vector    | `v.contains(&value)`             |
| `first()`        | Returns reference to the first element   | `v.first()`                      |
| `last()`         | Returns reference to the last element    | `v.last()`                       |
| `sort()`         | Sorts the vector in-place                | `v.sort()`                       |
| `reverse()`      | Reverses the order of elements           | `v.reverse()`                    |
| `iter()`         | Returns an iterator                      | `v.iter()`                       |

## Vectors with Custom Types

Vectors can store any type, including structs and enums:

```rust
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: String::from("Alice"), age: 30 },
    Person { name: String::from("Bob"), age: 25 },
    Person { name: String::from("Charlie"), age: 35 },
];
```

## Performance Considerations

- Appending elements to the end of a vector is generally fast (amortized O(1))
- Inserting or removing elements in the middle is slow (O(n))
- Vectors may need to reallocate when they grow beyond their capacity
- Using `Vec::with_capacity()` can improve performance if you know the approximate size

## Practice Exercise

Open the [`1_vectors.rs`](./1_vectors.rs) file and complete the exercises to test your understanding of vectors in Rust.

## Next Steps

Now that you understand vectors, you can proceed to [Strings](../02_strings/README.md), which are a specialized collection for handling text. 