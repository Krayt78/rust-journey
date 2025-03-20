// Arrays in Rust
//
// An array is a collection of elements of the same type.
// Arrays in Rust have a fixed length and all elements must be of the same type.
// They are stored on the stack, not the heap.

fn main() {
    println!("Exploring Arrays in Rust!");
    
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
    
    // Accessing array elements (using index notation)
    let first = numbers[0];
    let second = numbers[1];
    
    println!("\nAccessing array elements:");
    println!("First element: {}", first);
    println!("Second element: {}", second);
    
    // Getting the length of an array
    println!("\nArray length: {}", numbers.len());
    
    // Arrays are stored on the stack
    println!("Size of [i32; 5] array: {} bytes", std::mem::size_of::<[i32; 5]>());
    
    // Slicing arrays
    let slice = &numbers[1..4]; // This creates a slice from index 1 to 3 (not including 4)
    println!("\nSlice from index 1 to 3: {:?}", slice);
    
    // Iterating over arrays
    println!("\nIterating over array elements:");
    for num in numbers.iter() {
        println!("Value: {}", num);
    }
    
    // Iterating with index
    println!("\nIterating with index:");
    for (i, &num) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", i, num);
    }
    
    // Mutating arrays
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("\nOriginal mutable array: {:?}", mutable_array);
    
    mutable_array[2] = 10;
    println!("Modified mutable array: {:?}", mutable_array);
    
    // Arrays of different types
    let integers: [i32; 5] = [1, 2, 3, 4, 5];
    let floats: [f64; 3] = [1.1, 2.2, 3.3];
    let booleans: [bool; 2] = [true, false];
    let characters: [char; 4] = ['a', 'b', 'c', 'd'];
    
    println!("\nArrays of different types:");
    println!("Integers: {:?}", integers);
    println!("Floats: {:?}", floats);
    println!("Booleans: {:?}", booleans);
    println!("Characters: {:?}", characters);
    
    // Multi-dimensional arrays
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],    // Row 1
        [4, 5, 6],    // Row 2
    ];
    
    println!("\nMulti-dimensional array:");
    println!("Matrix: {:?}", matrix);
    println!("Element at row 1, column 2: {}", matrix[0][1]);
    
    // Passing arrays to functions
    let sum = sum_array(&numbers);
    println!("\nSum of array elements: {}", sum);
    
    // Sorting an array
    let mut unsorted = [5, 2, 8, 1, 9];
    println!("\nUnsorted array: {:?}", unsorted);
    unsorted.sort();
    println!("Sorted array: {:?}", unsorted);
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Function that takes an array reference as a parameter
fn sum_array(arr: &[i32; 5]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: These array declarations and operations have errors
    
    // Error 1: Wrong type in array
    let mixed_types = [1, 2, "three", 4, 5];
    
    // Error 2: Wrong size in type annotation
    let wrong_size: [i32; 3] = [1, 2, 3, 4, 5];
    
    // Error 3: Out of bounds access
    let small_array = [1, 2, 3];
    let out_of_bounds = small_array[3];
    
    // Error 4: Incorrect array initialization
    let repeated = [3, 5]; // Should be an array of five 3s
    
    // TODO: This function should calculate the average of an array of integers
    pub fn calculate_average(arr: &[i32; 5]) -> f64 {
        // This implementation has errors in how it calculates the average
        let mut sum = 0;
        for num in arr {
            sum += num; // Missing dereferencing
        }
        
        // Wrong division (missing type conversion)
        sum / arr.len()
    }
    
    // TODO: This function should find the maximum value in an array
    pub fn find_max(arr: &[i32; 5]) -> i32 {
        // This implementation has logic errors
        let mut max = arr[0];
        
        // Wrong loop implementation
        for i in 0..arr.len() {
            // Missing comparison
        }
        
        max
    }
    
    // TODO: This function should transpose a 2x3 matrix (convert rows to columns)
    pub fn transpose_matrix(matrix: [[i32; 3]; 2]) -> [[i32; 2]; 3] {
        // This implementation has logic errors
        let mut result = [[0; 2]; 3];
        
        // The loop logic here is incorrect
        for i in 0..2 {
            for j in 0..3 {
                result[i][j] = matrix[i][j]; // Wrong indices
            }
        }
        
        result
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning array challenges...");
        
        // Challenge 1: Fix array declarations and access
        challenge_array_basics()?;
        
        // Challenge 2: Fix the array averaging function
        let test_array = [10, 20, 30, 40, 50];
        let avg = challenges::calculate_average(&test_array);
        
        if (avg - 30.0).abs() > 0.0001 {
            return Err(format!("calculate_average returned wrong value. Expected: 30.0, Got: {}", avg));
        }
        println!("Array averaging function works correctly!");
        
        // Challenge 3: Fix the find max function
        let max = challenges::find_max(&[3, 1, 5, 2, 4]);
        if max != 5 {
            return Err(format!("find_max returned wrong value. Expected: 5, Got: {}", max));
        }
        println!("find_max function works correctly!");
        
        // Challenge 4: Fix the matrix transpose function
        let matrix = [
            [1, 2, 3],
            [4, 5, 6]
        ];
        
        let transposed = challenges::transpose_matrix(matrix);
        
        // Check first element of transposed matrix
        if transposed[0][0] != 1 || transposed[0][1] != 4 {
            return Err("transpose_matrix failed - first row is incorrect".to_string());
        }
        
        println!("Matrix transpose function works correctly!");
        
        println!("All array challenges completed successfully!");
        Ok(())
    }
    
    fn challenge_array_basics() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Array basics challenge completed!");
        Ok(())
    }
} 