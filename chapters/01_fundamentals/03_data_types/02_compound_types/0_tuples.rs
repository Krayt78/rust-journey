// Tuples in Rust
//
// A tuple is a collection of values of different types.
// Tuples have a fixed length and cannot be resized after declaration.
// They are useful when you want to return multiple values from a function
// or group related data together.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type

fn main() {
    println!("Exploring Tuples in Rust!");
    
    //------------------------------------------------------
    // BASIC TUPLE DECLARATIONS
    //------------------------------------------------------
    // Basic tuple declaration
    let tup = (500, 6.4, 1);
    println!("Basic tuple: {:?}", tup);
    
    // Tuple with explicit type annotation
    let tuple_with_types: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple with explicit types: {:?}", tuple_with_types);
    
    //------------------------------------------------------
    // ACCESSING TUPLE ELEMENTS
    //------------------------------------------------------
    // Accessing tuple elements with indexing (using the dot notation)
    let first = tuple_with_types.0;
    let second = tuple_with_types.1;
    let third = tuple_with_types.2;
    
    println!("\nAccessing tuple elements:");
    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Third: {}", third);
    
    //------------------------------------------------------
    // DESTRUCTURING TUPLES
    //------------------------------------------------------
    // Destructuring tuples
    let (x, y, z) = tuple_with_types;
    
    println!("\nDestructured tuple:");
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    
    //------------------------------------------------------
    // TUPLE STRUCTS
    //------------------------------------------------------
    // Tuple structs (named tuples)
    struct Color(i32, i32, i32);  // RGB color
    struct Point(i32, i32, i32);  // 3D point
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("\nTuple structs:");
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    //------------------------------------------------------
    // TUPLES AS FUNCTION RETURNS
    //------------------------------------------------------
    // Returning multiple values from a function using a tuple
    fn get_dimensions() -> (u32, u32, u32) {
        (1920, 1080, 32) // width, height, color depth
    }
    
    let dimensions = get_dimensions();
    println!("\nTuple from function:");
    println!("Dimensions: {}x{}x{} bits", dimensions.0, dimensions.1, dimensions.2);
    
    // Immediate destructuring of a function return
    let (width, height, depth) = get_dimensions();
    println!("Destructured dimensions: {}x{}x{} bits", width, height, depth);
    
    //------------------------------------------------------
    // UNIT TYPE (EMPTY TUPLE)
    //------------------------------------------------------
    // The unit type () is an empty tuple
    let unit_value = ();
    println!("\nUnit value size: {} bytes", std::mem::size_of_val(&unit_value));
    
    // Functions with no explicit return type return the unit value
    fn does_nothing() {}
    
    let nothing = does_nothing();
    println!("Functions with no return value return the unit type");
    
    // You can use a semicolon to make an expression return the unit type
    let unit_result = {
        let x = 5;
        x + 5; // Note the semicolon here
    };
    println!("Expression with trailing semicolon returns: {:?}", unit_result);
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These tuple declarations and operations have errors
    pub fn challenge_tuple_basics() -> Result<(), String> {
        // Error 1: Type mismatch in tuple declaration
        let person: (String, i32) = ("Alice", "29");
        
        // Error 2: Incorrect index for age
        let age = person.2;
        
        // Error 3: Wrong destructuring pattern (missing a field)
        let (name) = person;
        
        // Don't modify this verification
        if name != "Alice" || age != 29 {
            return Err(format!("Expected name 'Alice' and age 29, got '{}' and {}", name, age));
        }
        
        Ok(())
    }
    
    // TODO: This function should return a tuple with a person's info
    pub fn get_person_info() -> (String, u32, bool) {
        // This function should return a tuple with:
        // 1. Name as a String
        // 2. Age as a u32
        // 3. Is_student as a bool
        
        // The current implementation has errors
        let name = "Bob";
        let age = 20;
        let is_student = true;
        
        (name, age) // Missing the is_student field
    }
    
    // TODO: This function should swap two values
    pub fn swap_values(a: i32, b: i32) -> (i32, i32) {
        // This should return a tuple with the values swapped
        (a, b) // Not swapped
    }
    
    // TODO: This function should swap a and b if a < b, otherwise return as is
    pub fn swap_if_greater(a: i32, b: i32) -> (i32, i32) {
        // There's a logical error in this condition
        if a > b {
            (b, a) // Swapping when a is greater than b
        } else {
            (a, b)
        }
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning tuple challenges...");
        
        // Challenge 1: Fix tuple basics
        if let Err(e) = challenges::challenge_tuple_basics() {
            return Err(format!("Tuple basics challenge failed: {}", e));
        }
        println!("Successfully fixed the tuple declarations!");
        
        // Challenge 2: Fix get_person_info function
        let person = challenges::get_person_info();
        if person.0 != "Bob" || person.1 != 20 || !person.2 {
            return Err(format!(
                "get_person_info expected (\"Bob\", 20, true), got ({:?}, {}, {})",
                person.0, person.1, person.2
            ));
        }
        println!("Successfully fixed the get_person_info function!");
        
        // Challenge 3: Fix swap_values function
        let (a, b) = (10, 20);
        let (swapped_a, swapped_b) = challenges::swap_values(a, b);
        if swapped_a != b || swapped_b != a {
            return Err(format!(
                "swap_values({}, {}) should return ({}, {}), got ({}, {})",
                a, b, b, a, swapped_a, swapped_b
            ));
        }
        println!("Successfully fixed the swap_values function!");
        
        // Challenge 4: Fix swap_if_greater function
        let (x1, y1) = (5, 10);
        let (swapped_x1, swapped_y1) = challenges::swap_if_greater(x1, y1);
        if swapped_x1 != 10 || swapped_y1 != 5 {
            return Err(format!(
                "swap_if_greater({}, {}) should return ({}, {}), got ({}, {})",
                x1, y1, y1, x1, swapped_x1, swapped_y1
            ));
        }
        
        let (x2, y2) = (20, 10);
        let (swapped_x2, swapped_y2) = challenges::swap_if_greater(x2, y2);
        if swapped_x2 != 10 || swapped_y2 != 20 {
            return Err(format!(
                "swap_if_greater({}, {}) should return ({}, {}), got ({}, {})",
                x2, y2, y2, x2, swapped_x2, swapped_y2
            ));
        }
        println!("Successfully fixed the swap_if_greater function!");
        
        println!("All tuple challenges completed successfully!");
        Ok(())
    }
} 