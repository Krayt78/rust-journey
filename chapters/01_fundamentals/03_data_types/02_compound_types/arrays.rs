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
    
    // Challenge: Create a function that calculates the average of an array of integers
    // Your code here:
    
    // Challenge: Create a function that finds the largest element in an array
    // Your code here:
}

// Function that takes an array reference as a parameter
fn sum_array(arr: &[i32; 5]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
} 