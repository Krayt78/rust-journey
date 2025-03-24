// For Loops in Rust
//
// This program demonstrates how for loops work in Rust.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for

fn main() {
    println!("Exploring For Loops in Rust!");
    
    //------------------------------------------------------
    // FOR LOOP WITH ARRAY
    //------------------------------------------------------
    println!("\n--- For Loop with Array ---");
    
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    //------------------------------------------------------
    // FOR LOOP WITH RANGE
    //------------------------------------------------------
    println!("\n--- For Loop with Range ---");
    
    // Note: Ranges are exclusive by default (1..4 means 1, 2, 3)
    for number in 1..4 {
        println!("{}!", number);
    }
    
    println!("\n--- For Loop with Inclusive Range ---");
    
    // Use ..= for inclusive ranges
    for number in 1..=4 {
        println!("{}!", number);
    }
    
    //------------------------------------------------------
    // FOR LOOP WITH ENUMERATE
    //------------------------------------------------------
    println!("\n--- For Loop with Enumerate ---");
    
    let names = vec!["Alice", "Bob", "Charlie"];
    
    for (index, name) in names.iter().enumerate() {
        println!("{} is at index {}", name, index);
    }
    
    //------------------------------------------------------
    // FOR LOOP WITH ITERATORS
    //------------------------------------------------------
    println!("\n--- For Loop with Iterator Methods ---");
    
    // Using iterator adaptors like map
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("Doubling each number:");
    for doubled in numbers.iter().map(|n| n * 2) {
        println!("{}", doubled);
    }
    
    // Using filter
    println!("\nFiltering even numbers:");
    for even in numbers.iter().filter(|n| *n % 2 == 0) {
        println!("{}", even);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the for loop to properly iterate over the elements
    pub fn challenge_for_loop() -> Result<(), String> {
        let numbers = [10, 20, 30, 40, 50];
        let mut sum = 0;
        
        // This for loop should add up all the numbers, but there's an issue
        // Fix it to correctly sum all values
        for i in 0..3 {
            sum += numbers[i];
        }
        
        if sum != 150 {
            return Err(format!("Sum should be 150, but got {}", sum));
        }
        
        Ok(())
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning For Loop challenges...");
        
        // Challenge: Fix for loop
        if let Err(e) = challenges::challenge_for_loop() {
            return Err(format!("Challenge failed: {}", e));
        }
        println!("Successfully fixed the for loop!");
        
        println!("All For Loop challenges completed successfully!");
        Ok(())
    }
} 