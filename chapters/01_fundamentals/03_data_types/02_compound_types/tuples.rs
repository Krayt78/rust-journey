// Tuples in Rust
//
// A tuple is a collection of values of different types.
// Tuples have a fixed length and cannot be resized after declaration.
// They are useful when you want to return multiple values from a function
// or group related data together.

fn main() {
    println!("Exploring Tuples in Rust!");
    
    // Basic tuple declaration
    let tup = (500, 6.4, 1);
    println!("Basic tuple: {:?}", tup);
    
    // Tuple with explicit type annotation
    let tuple_with_types: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple with explicit types: {:?}", tuple_with_types);
    
    // Accessing tuple elements with indexing (using the dot notation)
    let first = tuple_with_types.0;
    let second = tuple_with_types.1;
    let third = tuple_with_types.2;
    
    println!("\nAccessing tuple elements:");
    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Third: {}", third);
    
    // Destructuring tuples
    let (x, y, z) = tuple_with_types;
    
    println!("\nDestructured tuple values:");
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    
    // Destructuring with placeholder '_' to ignore specific values
    let (a, _, c) = tuple_with_types;
    println!("\nPartially destructured tuple:");
    println!("a: {}", a);
    println!("c: {}", c);
    
    // Nested tuples
    let nested_tuple = ((1, 2), (3, 4), 5);
    println!("\nNested tuple: {:?}", nested_tuple);
    
    // Accessing elements in nested tuples
    let ((a, b), (c, d), e) = nested_tuple;
    println!("Destructured nested tuple: a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);
    
    // Tuple as a function return value
    let stats = get_stats();
    println!("\nFunction returned tuple: {:?}", stats);
    println!("Min: {}, Max: {}, Average: {}", stats.0, stats.1, stats.2);
    
    // Unit tuple 
    // An empty tuple () is called the "unit" type and represents an empty value
    let unit = ();
    println!("\nUnit tuple: {:?}", unit);
    println!("Size of unit tuple: {} bytes", std::mem::size_of_val(&unit));
    
    // Tuples can contain different types
    let mixed = ("hello", 42, true, 3.14);
    println!("\nMixed type tuple: {:?}", mixed);
    
    // Tuple with a single element needs a comma
    let single_element = (42,);  // Without the comma, this would just be an integer in parentheses
    println!("\nSingle element tuple: {:?}", single_element);
    
    // Creating a tuple to pass multiple values to a function
    print_coordinates((10, 20));
    
    // Challenge: Create a function that returns a tuple containing 
    // a string, an integer, and a boolean
    // Your code here:
    
    // Challenge: Create a tuple containing information about a person 
    // (name, age, height in meters) and destructure it to print a nice 
    // formatted string about the person
    // Your code here:
}

// Example function that returns a tuple
fn get_stats() -> (i32, i32, f64) {
    let min = 5;
    let max = 100;
    let average = 42.5;
    
    (min, max, average) // Return a tuple without 'return' keyword
}

// Function that takes a tuple as a parameter
fn print_coordinates(coord: (i32, i32)) {
    println!("\nCoordinates: x={}, y={}", coord.0, coord.1);
} 