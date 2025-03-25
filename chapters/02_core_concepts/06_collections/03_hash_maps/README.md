# Hash Maps in Rust

A hash map (`HashMap<K, V>`) is a collection that associates keys of type `K` with values of type `V` using a hashing function to determine how these keys and values are stored in memory.

## Creating Hash Maps

There are several ways to create a hash map:

### Using `HashMap::new()`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
```

### From iterators of key-value pairs

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Red")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

## Accessing Values

### Using the get method

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

match score {
    Some(value) => println!("Score: {}", value),
    None => println!("Team not found"),
}
```

### Using the index operator

```rust
let score = scores["Blue"]; // Panics if key doesn't exist
```

## Updating Hash Maps

### Inserting a Key-Value Pair

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);
```

### Overwriting a Value

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // Overwrites previous value
```

### Inserting Only If Key Doesn't Exist

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Only inserts if the key doesn't already exist
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50); // This won't change Blue's value
```

### Updating a Value Based on the Old Value

```rust
let text = "hello world wonderful world";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}
```

## Removing Entries

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

scores.remove(&String::from("Blue")); // Removes the Blue team
```

## Iterating Over Hash Maps

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Iterate over key-value pairs
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

## Ownership and Hash Maps

When values are inserted into a hash map, the hash map takes ownership of them if they implement the `Copy` trait. For values like strings that don't implement `Copy`, the values will be moved and the hash map will be the owner.

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// field_name and field_value are invalid at this point
```

## Performance and Customization

### Default Hashing Function

By default, `HashMap` uses a hashing algorithm (SipHash) that provides resistance against hash table collision attacks. It's not the fastest hashing algorithm, but it provides good security.

### Custom Hasher

You can use a different hasher by specifying a different `BuildHasher`:

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher;

type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

let mut map: FastMap<String, i32> = FastMap::default();
```

## Common Hash Map Methods

| Method             | Description                                         | Example                               |
|--------------------|-----------------------------------------------------|---------------------------------------|
| `insert()`         | Inserts a key-value pair into the map               | `map.insert("key", 42)`               |
| `get()`            | Returns an Option with a reference to the value     | `map.get(&"key")`                     |
| `get_mut()`        | Returns an Option with a mutable reference          | `map.get_mut(&"key")`                 |
| `contains_key()`   | Returns true if the map contains the key            | `map.contains_key(&"key")`            |
| `entry()`          | Returns an Entry for in-place manipulation          | `map.entry("key").or_insert(42)`      |
| `remove()`         | Removes a key from the map                          | `map.remove(&"key")`                  |
| `len()`            | Returns the number of elements                      | `map.len()`                           |
| `is_empty()`       | Returns true if the map contains no elements        | `map.is_empty()`                      |
| `clear()`          | Removes all elements from the map                   | `map.clear()`                         |
| `keys()`           | Returns an iterator over the keys                   | `map.keys()`                          |
| `values()`         | Returns an iterator over the values                 | `map.values()`                        |
| `values_mut()`     | Returns an iterator over mutable values             | `map.values_mut()`                    |
| `iter()`           | Returns an iterator over key-value pairs            | `map.iter()`                          |
| `iter_mut()`       | Returns an iterator over mutable key-value pairs    | `map.iter_mut()`                      |

## Use Cases for Hash Maps

Hash maps are ideal for:

1. Fast lookups by key
2. Caching computation results
3. Counting occurrences (as in the word count example)
4. Deduplication of data
5. Representing relationships between data

## Practice Exercise

Open the [`3_hash_maps.rs`](./3_hash_maps.rs) file and complete the exercises to test your understanding of hash maps in Rust.

## Next Steps

Congratulations! You've completed the chapter on Collections in Rust. You now have a solid understanding of the most commonly used collection types in Rust: vectors, strings, and hash maps. 

In the next chapters, you'll explore more advanced Rust features such as error handling, generics, and traits. 