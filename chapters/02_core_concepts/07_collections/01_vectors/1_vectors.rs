// Vectors in Rust
//
// This file demonstrates how to use vectors in Rust,
// which are resizable arrays that store elements of the same type.

fn main() {
    println!("Understanding Vectors in Rust!");
    
    //------------------------------------------------------
    // CREATING VECTORS
    //------------------------------------------------------
    println!("\n=== Creating Vectors ===");
    
    // Creating an empty vector
    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);
    
    // Creating a vector with initial values using the vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector with initial values: {:?}", v2);
    
    // Creating a vector with a specific capacity
    let mut v3: Vec<String> = Vec::with_capacity(5);
    println!("Vector with capacity 5: {:?}", v3);
    println!("Length: {}, Capacity: {}", v3.len(), v3.capacity());
    
    // Creating a vector with repeated values
    let v4 = vec![0; 10]; // Creates a vector with ten zeros
    println!("Vector with 10 zeros: {:?}", v4);
    
    //------------------------------------------------------
    // UPDATING VECTORS
    //------------------------------------------------------
    println!("\n=== Updating Vectors ===");
    
    // Adding elements to a vector
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("After pushing elements: {:?}", v);
    
    // Inserting at a specific position
    v.insert(1, 15);
    println!("After inserting 15 at index 1: {:?}", v);
    
    // Removing elements
    let removed = v.remove(2);
    println!("Removed {} at index 2: {:?}", removed, v);
    
    // Popping elements from the end
    let popped = v.pop();
    println!("Popped {:?} from the end: {:?}", popped, v);
    
    // Clearing a vector
    v.clear();
    println!("After clearing: {:?}", v);
    
    //------------------------------------------------------
    // ACCESSING ELEMENTS
    //------------------------------------------------------
    println!("\n=== Accessing Elements ===");
    
    let v = vec![10, 20, 30, 40, 50];
    
    // Accessing by index (panics if out of bounds)
    let third = v[2];
    println!("Third element: {}", third);
    
    // Safer access with get() method
    match v.get(2) {
        Some(value) => println!("Third element (safe): {}", value),
        None => println!("No element at index 2"),
    }
    
    match v.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("No element at index 10"),
    }
    
    // First and last elements
    if let Some(first) = v.first() {
        println!("First element: {}", first);
    }
    
    if let Some(last) = v.last() {
        println!("Last element: {}", last);
    }
    
    //------------------------------------------------------
    // ITERATING OVER VECTORS
    //------------------------------------------------------
    println!("\n=== Iterating Over Vectors ===");
    
    let v = vec![100, 200, 300, 400, 500];
    
    // Simple iteration
    println!("Simple iteration:");
    for element in &v {
        println!("  {}", element);
    }
    
    // Iteration with indices
    println!("Iteration with indices:");
    for (i, element) in v.iter().enumerate() {
        println!("  v[{}] = {}", i, element);
    }
    
    // Modifying elements during iteration
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Before modification: {:?}", v);
    
    for element in &mut v {
        *element *= 2; // Double each element
    }
    
    println!("After modification: {:?}", v);
    
    //------------------------------------------------------
    // VECTORS WITH DIFFERENT TYPES
    //------------------------------------------------------
    println!("\n=== Vectors with Different Types ===");
    
    // Vector of integers
    let integers = vec![1, 2, 3, 4, 5];
    println!("Integers: {:?}", integers);
    
    // Vector of strings
    let strings = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];
    println!("Strings: {:?}", strings);
    
    // Vector of custom structs
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
    ];
    
    println!("People:");
    for person in &people {
        println!("  {} is {} years old", person.name, person.age);
    }
    
    // Vector of mixed types using enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Sample")),
    ];
    
    println!("Mixed types using enum: {:?}", row);
    
    //------------------------------------------------------
    // COMMON VECTOR OPERATIONS
    //------------------------------------------------------
    println!("\n=== Common Vector Operations ===");
    
    let mut v = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    println!("Original vector: {:?}", v);
    
    // Sorting
    v.sort();
    println!("After sorting: {:?}", v);
    
    // Reversing
    v.reverse();
    println!("After reversing: {:?}", v);
    
    // Contains
    println!("Contains 3? {}", v.contains(&3));
    println!("Contains 10? {}", v.contains(&10));
    
    // Extending from another vector
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    v1.extend(v2); // v2 is moved here
    println!("After extending: {:?}", v1);
    
    // Creating a slice
    let v = vec![10, 20, 30, 40, 50];
    let slice = &v[1..4]; // Elements at indices 1, 2, and 3
    println!("Slice of vector: {:?}", slice);
    
    // Joining vectors
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = [v1, v2].concat();
    println!("Concatenated vector: {:?}", v3);
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Vector Transformation
//
// Implement the `double_values` function that takes a vector of integers
// and returns a new vector with each value doubled.

fn double_values(values: Vec<i32>) -> Vec<i32> {
    // TODO: Implement this function.
    // Create a new vector with each value from the input vector doubled.
    
    // Return the new vector
    Vec::new() // Replace this placeholder
}

// Challenge 2: Vector Filtering
//
// Implement the `filter_even_numbers` function that takes a vector of integers
// and returns a new vector containing only the even numbers.

fn filter_even_numbers(values: Vec<i32>) -> Vec<i32> {
    // TODO: Implement this function.
    // Create a new vector containing only the even numbers from the input vector.
    
    // Return the new vector
    Vec::new() // Replace this placeholder
}

// Challenge 3: Vector Statistics
//
// Implement the `calculate_statistics` function that takes a vector of integers
// and returns a tuple with the following statistics:
// (minimum, maximum, sum, average)
// If the vector is empty, return None.

fn calculate_statistics(values: &Vec<i32>) -> Option<(i32, i32, i32, f64)> {
    // TODO: Implement this function.
    // Calculate the minimum, maximum, sum, and average of the values in the vector.
    // Return None if the vector is empty.
    
    None // Replace this placeholder
}

// Challenge 4: Working with Custom Types
//
// Implement the `find_oldest_person` function that takes a vector of Person structs
// and returns a reference to the oldest person. If the vector is empty, return None.

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn find_oldest_person(people: &Vec<Person>) -> Option<&Person> {
    // TODO: Implement this function.
    // Find and return a reference to the person with the highest age.
    // Return None if the vector is empty.
    
    None // Replace this placeholder
}

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_double_values() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![2, 4, 6, 8, 10];
        assert_eq!(double_values(input), expected);
    }
    
    #[test]
    fn test_filter_even_numbers() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = vec![2, 4, 6, 8, 10];
        assert_eq!(filter_even_numbers(input), expected);
    }
    
    #[test]
    fn test_calculate_statistics() {
        let input = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
        let expected = Some((1, 9, 45, 5.0));
        assert_eq!(calculate_statistics(&input), expected);
        
        let empty: Vec<i32> = Vec::new();
        assert_eq!(calculate_statistics(&empty), None);
    }
    
    #[test]
    fn test_find_oldest_person() {
        let people = vec![
            Person { name: String::from("Alice"), age: 30 },
            Person { name: String::from("Bob"), age: 25 },
            Person { name: String::from("Charlie"), age: 35 },
            Person { name: String::from("Dave"), age: 20 },
        ];
        
        let oldest = find_oldest_person(&people);
        assert!(oldest.is_some());
        if let Some(person) = oldest {
            assert_eq!(person.name, "Charlie");
            assert_eq!(person.age, 35);
        }
        
        let empty: Vec<Person> = Vec::new();
        assert!(find_oldest_person(&empty).is_none());
    }
} 