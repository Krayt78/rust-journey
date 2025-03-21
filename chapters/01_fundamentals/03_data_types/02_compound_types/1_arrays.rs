// Arrays in Rust
//
// An array is a collection of elements of the same type.
// Arrays in Rust have a fixed length and all elements must be of the same type.
// They are stored on the stack, not the heap.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type

fn main() {
    println!("Exploring Arrays in Rust!");
    
    //------------------------------------------------------
    // BASIC ARRAY DECLARATIONS
    //------------------------------------------------------
    // Basic array declaration
    let numbers = [1, 2, 3, 4, 5];
    println!("Basic array: {:?}", numbers);
    
    // Array with explicit type annotation
    let numbers_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array with explicit type: {:?}", numbers_with_type);
    
    // Creating an array with repeated values
    // This creates an array of size 5 with all elements set to 3
    let repeated = [3; 5]; // equivalent to [3, 3, 3, 3, 3]
    println!("Array with repeated values: {:?}", repeated);
    
    //------------------------------------------------------
    // ACCESSING ARRAY ELEMENTS
    //------------------------------------------------------
    // Accessing array elements (using index notation)
    let first = numbers[0];
    let second = numbers[1];
    
    println!("\nAccessing array elements:");
    println!("First element: {}", first);
    println!("Second element: {}", second);
    
    //------------------------------------------------------
    // ARRAY PROPERTIES
    //------------------------------------------------------
    // Getting array length
    let length = numbers.len();
    println!("\nArray length: {}", length);
    
    // Getting size of array in bytes
    let size = std::mem::size_of_val(&numbers);
    println!("Array size in bytes: {} (5 elements * 4 bytes each for i32)", size);
    
    //------------------------------------------------------
    // ARRAY ITERATION
    //------------------------------------------------------
    // Iterating over array elements
    println!("\nIterating over array:");
    
    // Using a for loop
    for element in numbers.iter() {
        println!("Element: {}", element);
    }
    
    // Using a for loop with index
    println!("\nIterating with index:");
    for (i, &element) in numbers.iter().enumerate() {
        println!("Element at index {}: {}", i, element);
    }
    
    //------------------------------------------------------
    // ARRAY SLICES
    //------------------------------------------------------
    // Creating a slice (a reference to a portion of an array)
    let slice = &numbers[1..4]; // Elements at indices 1, 2, and 3
    println!("\nSlice of the array: {:?}", slice);
    
    // Slice with inclusive range
    let inclusive_slice = &numbers[1..=3]; // Elements at indices 1, 2, and 3
    println!("Slice with inclusive range: {:?}", inclusive_slice);
    
    //------------------------------------------------------
    // ARRAY METHODS
    //------------------------------------------------------
    // Using array methods
    let sum: i32 = numbers.iter().sum();
    println!("\nSum of all elements: {}", sum);
    
    // Finding the maximum value
    if let Some(&max) = numbers.iter().max() {
        println!("Maximum value: {}", max);
    }
    
    // Finding the minimum value
    if let Some(&min) = numbers.iter().min() {
        println!("Minimum value: {}", min);
    }
    
    //------------------------------------------------------
    // MULTI-DIMENSIONAL ARRAYS
    //------------------------------------------------------
    // Multi-dimensional arrays
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    
    println!("\nMatrix (multi-dimensional array):");
    for row in &matrix {
        println!("{:?}", row);
    }
    
    // Accessing elements in a multi-dimensional array
    let element = matrix[1][2]; // Row 1, Column 2
    println!("Element at matrix[1][2]: {}", element);
    
    println!("\nMultidimensional array:");
    println!("First row: {:?}", matrix[0]);
    println!("Second element of first row: {}", matrix[0][1]);
    println!("First element of second row: {}", matrix[1][0]);
    println!("Second element of third row: {}", matrix[2][1]);
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These array declarations and operations have errors
    pub fn challenge_array_basics() -> Result<(), String> {
        // Error 1: Array size mismatch in declaration
        let numbers: [i32; 4] = [1, 2, 3, 4, 5];
        
        // Error 2: Invalid index access
        let last = numbers[5];
        
        // Error 3: Attempting to change array length
        numbers.push(6);
        
        // Don't modify this verification
        if numbers.len() != 5 || last != 5 {
            return Err("Array challenges not fixed correctly".to_string());
        }
        
        Ok(())
    }
    
    // TODO: This function should return an array with n elements, all initialized to value
    pub fn create_array(value: i32, n: usize) -> [i32; 5] {
        // Problem: This function should work for any size n, but the return type 
        // is fixed to size 5, which is incorrect
        let array = [value; n];
        array
    }
    
    // TODO: This function should find the average of an array
    pub fn find_average(arr: &[i32]) -> f64 {
        // This implementation has errors
        let sum = 0;
        for i in 0..arr.len() {
            sum += arr[i];
        }
        
        sum / arr.len()
    }
    
    // TODO: This function should reverse an array
    pub fn reverse_array(arr: [i32; 5]) -> [i32; 5] {
        // This implementation doesn't correctly reverse the array
        let mut result = [0; 5];
        for i in 0..arr.len() {
            result[i] = arr[i];
        }
        result
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning array challenges...");
        
        // Challenge 1: Fix array basics
        if let Err(e) = challenges::challenge_array_basics() {
            return Err(format!("Array basics challenge failed: {}", e));
        }
        println!("Successfully fixed array basics!");
        
        // Challenge 2: Fix create_array function
        let test_size = 3;
        let test_value = 7;
        let result = challenges::create_array(test_value, test_size);
        
        if result.len() != test_size {
            return Err(format!(
                "create_array({}, {}) should return an array of length {}, got length {}",
                test_value, test_size, test_size, result.len()
            ));
        }
        
        for &val in &result {
            if val != test_value {
                return Err(format!(
                    "create_array({}, {}) should fill array with {}, got {}",
                    test_value, test_size, test_value, val
                ));
            }
        }
        println!("Successfully fixed create_array function!");
        
        // Challenge 3: Fix find_average function
        let test_array = [10, 20, 30, 40, 50];
        let avg = challenges::find_average(&test_array);
        if (avg - 30.0).abs() > 0.001 {
            return Err(format!(
                "find_average([10, 20, 30, 40, 50]) should return 30.0, got {}",
                avg
            ));
        }
        println!("Successfully fixed find_average function!");
        
        // Challenge 4: Fix reverse_array function
        let test_array = [1, 2, 3, 4, 5];
        let reversed = challenges::reverse_array(test_array);
        let expected = [5, 4, 3, 2, 1];
        
        if reversed != expected {
            return Err(format!(
                "reverse_array({:?}) should return {:?}, got {:?}",
                test_array, expected, reversed
            ));
        }
        println!("Successfully fixed reverse_array function!");
        
        println!("All array challenges completed successfully!");
        Ok(())
    }
} 