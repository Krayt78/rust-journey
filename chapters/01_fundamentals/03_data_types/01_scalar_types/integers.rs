// Integers in Rust
//
// Rust has several integer types with different sizes and signedness.
// The naming convention is: 
// - Signed integers: i8, i16, i32, i64, i128, isize
// - Unsigned integers: u8, u16, u32, u64, u128, usize
//
// The number after i or u indicates how many bits are used to store the number.

fn main() {
    println!("Let's explore integer types in Rust!");
    
    // Signed integers can store both positive and negative values
    let a: i8 = 127;    // Maximum value for i8
    let b: i8 = -128;   // Minimum value for i8
    println!("i8 range: {} to {}", b, a);
    
    // Unsigned integers can only store non-negative values
    let c: u8 = 255;    // Maximum value for u8
    let d: u8 = 0;      // Minimum value for u8
    println!("u8 range: {} to {}", d, c);
    
    // The default integer type in Rust is i32
    let e = 42;   // Type is inferred as i32
    println!("Default integer: {}", e);
    
    // isize and usize depend on the architecture of the computer
    // (64 bits on a 64-bit system, 32 bits on a 32-bit system)
    let f: isize = 9000;
    let g: usize = 9000;
    println!("isize: {}, usize: {}", f, g);
    
    // Integer literals can have a suffix to specify their type
    let h = 42u8;    // u8
    let i = 1_000i64; // i64 - note the underscore for readability
    println!("Explicit types: {} (u8), {} (i64)", h, i);
    
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
    
    // Challenge: Try creating variables with different integer types and perform operations
    // Your code here:
    
    // Challenge: Try creating a value that's just at the boundary of an integer type
    // Your code here:
}