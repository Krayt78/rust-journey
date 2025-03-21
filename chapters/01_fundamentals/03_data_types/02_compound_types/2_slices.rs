// Slices in Rust
//
// A slice is a reference to a contiguous sequence of elements in a collection.
// Slices don't own data—they borrow it. They're useful when you want to
// reference only a portion of a collection.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch04-03-slices.html

fn main() {
    println!("Exploring Slices in Rust!");
    
    //------------------------------------------------------
    // BASIC SLICE CREATION
    //------------------------------------------------------
    // Creating a slice from an array
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4]; // This is a slice of the 2nd, 3rd, and 4th elements
    
    println!("Original array: {:?}", array);
    println!("Slice (1..4): {:?}", slice);
    
    // Slices have a length
    println!("\nSlice length: {}", slice.len());
    
    //------------------------------------------------------
    // ACCESSING SLICE ELEMENTS
    //------------------------------------------------------
    // Accessing elements in a slice
    println!("\nAccessing slice elements:");
    println!("First element: {}", slice[0]); // This is the 2nd element of the original array
    println!("Last element: {}", slice[slice.len() - 1]);
    
    //------------------------------------------------------
    // SLICE RANGES
    //------------------------------------------------------
    // Creating a slice of the entire array
    let full_slice = &array[..]; // Equivalent to &array[0..array.len()]
    println!("\nFull slice: {:?}", full_slice);
    
    // Slices with omitted bounds
    let from_start = &array[..3]; // Equivalent to &array[0..3]
    let to_end = &array[2..]; // Equivalent to &array[2..array.len()]
    
    println!("Slice from start to index 3: {:?}", from_start);
    println!("Slice from index 2 to end: {:?}", to_end);
    
    //------------------------------------------------------
    // INCLUSIVE RANGES
    //------------------------------------------------------
    // Using inclusive ranges
    let inclusive = &array[1..=3]; // Includes indices 1, 2, and 3
    println!("\nInclusive slice (1..=3): {:?}", inclusive);
    
    //------------------------------------------------------
    // STRING SLICES
    //------------------------------------------------------
    // String slices
    let message = "Hello, Rust!";
    let greeting = &message[0..5]; // "Hello"
    let language = &message[7..11]; // "Rust"
    
    println!("\nString slices:");
    println!("Original message: {}", message);
    println!("Greeting: {}", greeting);
    println!("Language: {}", language);
    
    //------------------------------------------------------
    // MUTABLE SLICES
    //------------------------------------------------------
    // Mutable slices
    let mut numbers = [1, 2, 3, 4, 5];
    let slice_mut = &mut numbers[1..4];
    
    println!("\nBefore modification:");
    println!("Mutable slice: {:?}", slice_mut);
    
    // Modifying the slice (which modifies the original array)
    slice_mut[0] = 20; // This changes the 2nd element of the original array
    slice_mut[1] = 30; // This changes the 3rd element of the original array
    
    println!("After modification:");
    println!("Mutable slice: {:?}", slice_mut);
    println!("Original array: {:?}", numbers);
    
    //------------------------------------------------------
    // SLICE METHODS
    //------------------------------------------------------
    // Using slice methods
    let numbers = [10, 20, 30, 40, 50];
    let slice = &numbers[..];
    
    println!("\nSlice methods:");
    println!("First element: {:?}", slice.first());
    println!("Last element: {:?}", slice.last());
    println!("Is empty: {}", slice.is_empty());
    
    // Getting a subslice
    let sub = &slice[1..3];
    println!("Subslice: {:?}", sub);
    
    //------------------------------------------------------
    // SLICES AS FUNCTION PARAMETERS
    //------------------------------------------------------
    // Passing slices to functions
    let sum = sum_slice(&numbers[1..4]);
    println!("\nSum of slice [20, 30, 40]: {}", sum);
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Function that takes a slice as a parameter
fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &n in slice {
        sum += n;
    }
    sum
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These slice operations have errors
    pub fn challenge_slice_basics() -> Result<(), String> {
        // Error 1: Incorrect slice creation
        let array = [10, 20, 30, 40, 50];
        let slice = array[1..3]; // Missing the reference operator
        
        // Error 2: Out of bounds slice
        let out_of_bounds = &array[3..10];
        
        // Error 3: Modifying a non-mutable slice
        let s = &array[1..4];
        s[0] = 100;
        
        // Don't modify below this line
        Ok(())
    }
    
    // TODO: This function should return a slice containing only positive numbers
    pub fn positive_numbers(numbers: &[i32]) -> &[i32] {
        // This implementation incorrectly tries to create a new slice
        let mut result = [];
        for &n in numbers {
            if n > 0 {
                result.push(n); // Can't modify the length of an array
            }
        }
        &result
    }
    
    // TODO: This function should find a substring within a string
    pub fn find_substring<'a>(text: &'a str, start_idx: usize, length: usize) -> &'a str {
        // This has an error in how the slice is created
        let end_idx = start_idx + length;
        &text[start_idx..=end_idx] // Using inclusive range, which might be out of bounds
    }
    
    // TODO: This function should safely access an element in a slice
    pub fn get_element(slice: &[i32], index: usize) -> i32 {
        // This implementation doesn't handle out-of-bounds safely
        slice[index] // This will panic if index is out of bounds
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning slice challenges...");
        
        // Challenge 1: Fix slice basics
        if let Err(e) = challenges::challenge_slice_basics() {
            return Err(format!("Slice basics challenge failed: {}", e));
        }
        println!("Successfully fixed slice basics!");
        
        // Challenge 2: Fix positive_numbers function
        let numbers = [-3, -1, 0, 1, 2, 5];
        let positives = challenges::positive_numbers(&numbers);
        let expected = &numbers[3..6]; // [1, 2, 5]
        
        if positives != expected {
            return Err(format!(
                "positive_numbers({:?}) should return {:?}, got {:?}",
                numbers, expected, positives
            ));
        }
        println!("Successfully fixed positive_numbers function!");
        
        // Challenge 3: Fix find_substring function
        let text = "Hello, Rust programmer!";
        let rust = challenges::find_substring(text, 7, 4);
        if rust != "Rust" {
            return Err(format!(
                "find_substring({:?}, 7, 4) should return \"Rust\", got {:?}",
                text, rust
            ));
        }
        println!("Successfully fixed find_substring function!");
        
        // Challenge 4: Fix get_element function
        let numbers = [10, 20, 30, 40, 50];
        
        // Test valid index
        let element = challenges::get_element(&numbers, 2);
        if element != 30 {
            return Err(format!(
                "get_element({:?}, 2) should return 30, got {}",
                numbers, element
            ));
        }
        
        // Test out-of-bounds index (should return a default value, not panic)
        let out_of_bounds = challenges::get_element(&numbers, 10);
        if out_of_bounds != 0 {
            return Err(format!(
                "get_element({:?}, 10) should return 0 for out-of-bounds, got {}",
                numbers, out_of_bounds
            ));
        }
        println!("Successfully fixed get_element function!");
        
        println!("All slice challenges completed successfully!");
        Ok(())
    }
} 