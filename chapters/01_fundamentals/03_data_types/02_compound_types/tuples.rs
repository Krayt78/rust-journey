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
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
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

// CHALLENGES: TODO: Fix the broken code in this module
mod challenges {
    // TODO: These tuple declarations and operations have errors
    
    // Error 1: Missing comma for single-element tuple
    let single_element = (42);
    
    // Error 2: Wrong type annotation
    let wrong_type: (i32, i32) = (42, "hello");
    
    // Error 3: Incorrect index access
    let tuple = (1, 2, 3);
    let third = tuple[2]; // Arrays use [index], tuples use .index
    
    // Error 4: Wrong destructuring pattern
    let (a, b) = (1, 2, 3);
    
    // TODO: This function should return a tuple with a person's info
    pub fn get_person_info() -> (String, u32, f64) {
        let name = "John Doe";
        let age = 30;
        let height = 1.85;
        
        // This return statement has an error
        (name, age, height) // Missing type conversion for &str to String
    }
    
    // TODO: This function should swap two values
    pub fn swap_values(a: i32, b: i32) -> (i32, i32) {
        // This has a logic error in how it returns the values
        (a, b)
    }
    
    // TODO: This function should swap a and b if a < b, otherwise return as is
    pub fn swap_if_greater(a: i32, b: i32) -> (i32, i32) {
        // This implementation has a logical error
        if a < b {
            (a, b)
        } else {
            (a, b)
        }
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning tuple challenges...");
        
        // Challenge 1: Fix tuple declarations and access
        challenge_tuple_basics()?;
        
        // Challenge 2: Fix the person info function
        let person_info = challenges::get_person_info();
        println!("Person info: {:?}", person_info);
        
        if person_info.0 != "John Doe" || person_info.1 <= 0 {
            return Err("get_person_info should return a tuple with name and positive age".to_string());
        }
        
        // Challenge 3: Fix the swap function
        let swapped = challenges::swap_values(10, 20);
        if swapped.0 != 20 || swapped.1 != 10 {
            return Err(format!("swap_values(10, 20) should return (20, 10), got {:?}", swapped));
        }
        println!("swap_values function works correctly!");
        
        // Challenge 4: Fix the swap_if_greater function
        let result = challenges::swap_if_greater(5, 10);
        if result.0 != 10 || result.1 != 5 {
            return Err("swap_if_greater(5, 10) should swap values".to_string());
        }
        let no_swap = challenges::swap_if_greater(10, 5);
        if no_swap.0 != 10 || no_swap.1 != 5 {
            return Err("swap_if_greater(10, 5) should not swap values".to_string());
        }
        println!("swap_if_greater function works correctly!");
        
        println!("All tuple challenges completed successfully!");
        Ok(())
    }
    
    fn challenge_tuple_basics() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Tuple basics challenge completed!");
        Ok(())
    }
} 