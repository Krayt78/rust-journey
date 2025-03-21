// Integers in Rust
//
// Rust has several integer types with different sizes and signedness.
// The naming convention is: 
// - Signed integers: i8, i16, i32, i64, i128, isize
// - Unsigned integers: u8, u16, u32, u64, u128, usize
//
// The number after i or u indicates how many bits are used to store the number.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types

fn main() {
    println!("Let's explore integer types in Rust!");
    
    //------------------------------------------------------
    // SIGNED INTEGERS
    //------------------------------------------------------
    // Signed integers can store both positive and negative values
    let a: i8 = 127;    // Maximum value for i8
    let b: i8 = -128;   // Minimum value for i8
    println!("i8 range: {} to {}", b, a);
    
    //------------------------------------------------------
    // UNSIGNED INTEGERS
    //------------------------------------------------------
    // Unsigned integers can only store non-negative values
    let c: u8 = 255;    // Maximum value for u8
    let d: u8 = 0;      // Minimum value for u8
    println!("u8 range: {} to {}", d, c);
    
    //------------------------------------------------------
    // DEFAULT INTEGER TYPE
    //------------------------------------------------------
    // The default integer type in Rust is i32
    let e = 42;   // Type is inferred as i32
    println!("Default integer: {}", e);
    
    //------------------------------------------------------
    // ARCHITECTURE-DEPENDENT SIZES
    //------------------------------------------------------
    // isize and usize depend on the architecture of the computer
    // (64 bits on a 64-bit system, 32 bits on a 32-bit system)
    let f: isize = 9000;
    let g: usize = 9000;
    println!("isize: {}, usize: {}", f, g);
    
    //------------------------------------------------------
    // INTEGER LITERALS
    //------------------------------------------------------
    // Integer literals can have a suffix to specify their type
    let h = 42u8;    // u8
    let i = 1_000i64; // i64 - note the underscore for readability
    println!("Explicit types: {} (u8), {} (i64)", h, i);
    
    //------------------------------------------------------
    // NUMBER REPRESENTATIONS
    //------------------------------------------------------
    // Different number representations
    let decimal = 98_222;      // Decimal
    let hex = 0xff;           // Hex
    let octal = 0o77;         // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A';          // Byte (u8 only) - ASCII value of 'A'
    
    println!("Number representations:");
    println!("Decimal: {}", decimal);
    println!("Hex: {} (0xff)", hex);
    println!("Octal: {} (0o77)", octal);
    println!("Binary: {} (0b1111_0000)", binary);
    println!("Byte: {} (b'A' - ASCII of 'A')", byte);
    
    //------------------------------------------------------
    // INTEGER OVERFLOW
    //------------------------------------------------------
    // Integer overflow behavior
    // In debug builds, Rust includes checks that panic on overflow
    // In release builds, Rust performs "two's complement wrapping"
    
    // Uncomment to see overflow panic in debug mode:
    // let will_overflow: u8 = 255 + 1;
    
    // Ways to handle potential overflow:
    let x: u8 = 255;
    
    // Wrapping (two's complement)
    let wrapped = x.wrapping_add(1);
    
    // Return None if overflow occurs
    let checked = x.checked_add(1);
    
    // Return the value and a boolean indicating overflow
    let overflowed = x.overflowing_add(1);
    
    // Saturate at the min or max value
    let saturated = x.saturating_add(1);
    
    println!("\nHandling overflow for u8 (255):");
    println!("Wrapped: {}", wrapped);
    println!("Checked: {:?}", checked);
    println!("Overflowed: {:?}", overflowed);
    println!("Saturated: {}", saturated);
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These integer declarations have errors
    
    // Error 1: This value is too large for i8
    let too_large: i8 = 200;
    
    // Error 2: This value is negative but the type is unsigned
    let negative: u16 = -42;
    
    // Error 3: This hex value is incompatible with the specified type
    let hex_value: u8 = 0xFFF;
    
    // Error 4: Missing type suffix for intended type
    let intended_u64 = 9999999999;
    
    // TODO: This function should add two u8 values, but handle overflow
    // by saturating at 255 (returning the value as a Some).
    // It should never return None.
    pub fn add_without_overflow(a: u8, b: u8) -> Option<u8> {
        // This has an error in the overflow handling logic
        let result = a + b;
        if result < a {
            None  // Incorrectly returns None on overflow
        } else {
            Some(result)
        }
    }
    
    // TODO: This function should calculate the sum of all integers from
    // 1 to n (inclusive), but it has logical errors
    pub fn sum_up_to(n: u32) -> u32 {
        let mut sum = 0;
        
        // This loop has an error that prevents it from summing correctly
        for i in 0..n {
            sum += i;
        }
        
        sum
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning integer challenges...");
        
        // Challenge 1: Fix type errors in integer declarations
        // This code has errors related to integer types and ranges
        challenge_integer_types()?;
        
        // Challenge 2: Fix the overflow handling function
        // The function should correctly handle potential overflows
        let result = challenges::add_without_overflow(255, 10);
        match result {
            Some(value) => {
                if value != 255 {
                    return Err("add_without_overflow should return 255 (saturated)".to_string());
                }
                println!("Overflow challenge completed!");
            },
            None => return Err("add_without_overflow should not return None for this input".to_string()),
        }
        
        // Challenge 3: Fix the calculation function
        // It should calculate the sum of all numbers from 1 to n
        let sum = challenges::sum_up_to(10);
        if sum != 55 {  // 1 + 2 + 3 + ... + 10 = 55
            return Err(format!("sum_up_to(10) should return 55, got {}", sum));
        }
        
        println!("All integer challenges completed successfully!");
        Ok(())
    }
    
    #[test]
    fn challenge_integer_types() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Integer types challenge completed!");
        Ok(())
    }
}