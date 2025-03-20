// Slices in Rust
//
// A slice is a reference to a contiguous sequence of elements in a collection.
// Slices don't own data—they borrow it. They're useful when you want to
// reference only a portion of a collection.

fn main() {
    println!("Exploring Slices in Rust!");
    
    // Creating a slice from an array
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4]; // This is a slice of the 2nd, 3rd, and 4th elements
    
    println!("Original array: {:?}", array);
    println!("Slice (1..4): {:?}", slice);
    
    // Slices have a length
    println!("\nSlice length: {}", slice.len());
    
    // Accessing elements in a slice
    println!("\nAccessing slice elements:");
    println!("First element: {}", slice[0]); // This is the 2nd element of the original array
    println!("Last element: {}", slice[slice.len() - 1]);
    
    // Creating a slice of the entire array
    let full_slice = &array[..]; // Equivalent to &array[0..array.len()]
    println!("\nFull slice: {:?}", full_slice);
    
    // Slices with omitted bounds
    let from_start = &array[..3]; // Equivalent to &array[0..3]
    let to_end = &array[2..];     // Equivalent to &array[2..array.len()]
    
    println!("\nSlice from start (..3): {:?}", from_start);
    println!("Slice to end (2..): {:?}", to_end);
    
    // Iterating over slices
    println!("\nIterating over slice elements:");
    for &item in slice.iter() {
        println!("Value: {}", item);
    }
    
    // String slices
    let message = "Hello, world!";
    let hello = &message[0..5];
    let world = &message[7..12];
    
    println!("\nString: {}", message);
    println!("Slice 'hello': {}", hello);
    println!("Slice 'world': {}", world);
    
    // Mutable slices
    let mut numbers = [1, 2, 3, 4, 5];
    println!("\nOriginal mutable array: {:?}", numbers);
    
    // Creating a mutable slice
    let mutable_slice = &mut numbers[1..4];
    println!("Mutable slice before: {:?}", mutable_slice);
    
    // Modifying elements through the slice
    mutable_slice[0] = 20; // This changes the 2nd element of the original array
    mutable_slice[1] = 30; // This changes the 3rd element of the original array
    
    println!("Mutable slice after: {:?}", mutable_slice);
    println!("Modified array: {:?}", numbers);
    
    // Slice type annotation
    let typed_slice: &[i32] = &array[1..4];
    println!("\nTyped slice: {:?}", typed_slice);
    
    // Function that takes a slice
    let sum = sum_slice(&array[1..4]);
    println!("\nSum of slice elements: {}", sum);
    
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
    for &item in slice {
        sum += item;
    }
    sum
}

// CHALLENGES: TODO: Fix the broken code in this module
mod challenges {
    // TODO: These slice operations have errors
    
    // Error 1: Incorrect slice syntax
    let array = [10, 20, 30, 40, 50];
    let slice1 = array[1, 3];
    
    // Error 2: Out of bounds slice
    let slice2 = &array[3..6];
    
    // Error 3: Mutable/immutable borrow confusion
    let mut array2 = [1, 2, 3];
    let slice3 = &array2[0..2];
    array2[0] = 10; // Error: cannot borrow as mutable because it's also borrowed as immutable
    println!("Slice: {:?}", slice3);
    
    // Error 4: Incorrect type handling
    let string = "hello";
    let char_slice: &[char] = &string[0..3];
    
    // TODO: This function should sum all values in a slice
    pub fn sum_slice(slice: &[i32]) -> i32 {
        // This implementation has errors in how it sums the values
        let sum = 0;
        
        // Incorrect loop handling
        for i in 0..slice.len() {
            sum += slice[i]; // Missing mutability
        }
        
        sum
    }
    
    // TODO: Find the starting position of a subslice within a slice, if it exists
    pub fn find_subslice(haystack: &[i32], needle: &[i32]) -> Option<usize> {
        // This implementation has logic errors
        if needle.len() > haystack.len() {
            return None;
        }
        
        for i in 0..=haystack.len() {  // Incorrect upper bound
            let mut found = true;
            
            for j in 0..needle.len() {
                if i + j > haystack.len() || haystack[i + j] != needle[j] {
                    found = false;
                    break;
                }
            }
            
            if found {
                return Some(i);
            }
        }
        
        None
    }
    
    // TODO: Return the middle slice of an array (excluding first and last elements)
    pub fn get_middle_slice(arr: &[i32]) -> &[i32] {
        // This implementation has errors in how it gets the middle slice
        &arr[0..arr.len()]  // Wrong indices
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning slice challenges...");
        
        // Challenge 1: Fix slice creation and access
        challenge_slice_basics()?;
        
        // Challenge 2: Fix the sum function
        let test_slice = &[1, 2, 3, 4, 5][..];
        let sum_result = challenges::sum_slice(test_slice);
        
        if sum_result != 15 {
            return Err(format!("sum_slice returned wrong value. Expected: 15, Got: {}", sum_result));
        }
        println!("Slice sum function works correctly!");
        
        // Challenge 3: Fix the find subslice function
        let haystack = [1, 2, 3, 4, 5, 6, 7, 8];
        let needle = [3, 4, 5];
        
        let found_at = challenges::find_subslice(&haystack, &needle);
        if found_at != Some(2) {
            return Err(format!("find_subslice returned wrong value. Expected: Some(2), Got: {:?}", found_at));
        }
        println!("find_subslice function works correctly!");
        
        // Challenge 4: Fix the middle slice function
        let array = [1, 2, 3, 4, 5];
        let middle = challenges::get_middle_slice(&array);
        
        if middle != [2, 3, 4] {
            return Err(format!("get_middle_slice returned wrong slice. Expected: [2, 3, 4], Got: {:?}", middle));
        }
        println!("get_middle_slice function works correctly!");
        
        println!("All slice challenges completed successfully!");
        Ok(())
    }
    
    fn challenge_slice_basics() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Slice basics challenge completed!");
        Ok(())
    }
} 