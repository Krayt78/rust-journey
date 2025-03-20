// Floating-Point Numbers in Rust
//
// Rust has two floating-point types:
// - f32: 32-bit floating point (single precision)
// - f64: 64-bit floating point (double precision)
//
// The default floating-point type is f64.

fn main() {
    println!("Exploring Floating-Point Numbers in Rust!");
    
    // Basic f64 (default) floating-point numbers
    let x = 2.0; // f64 by default
    println!("x (f64): {}", x);
    
    // Explicit f32
    let y: f32 = 3.0;
    println!("y (f32): {}", y);
    
    // Scientific notation
    let large_number = 1.2e4; // 1.2 × 10^4 = 12,000
    let small_number = 1.2e-4; // 1.2 × 10^-4 = 0.00012
    println!("Scientific notation: {} and {}", large_number, small_number);
    
    // Basic arithmetic with floating-point numbers
    let sum = 5.0 + 10.0;     // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4.0 * 30.0;    // Multiplication
    let quotient = 56.7 / 32.2;  // Division
    let remainder = 43.5 % 5.0;  // Remainder/Modulo
    
    println!("\nBasic arithmetic:");
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
    
    // Precision limitations
    let precision_example = 0.1 + 0.2;
    println!("\nPrecision example:");
    println!("0.1 + 0.2 = {}", precision_example);
    println!("Is 0.1 + 0.2 == 0.3? {}", precision_example == 0.3);
    
    // This happens because floating-point numbers are represented in binary,
    // and some decimal fractions can't be represented exactly in binary.
    
    // Special floating-point values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN; // Not a Number
    
    println!("\nSpecial values:");
    println!("Infinity: {}", infinity);
    println!("Negative Infinity: {}", neg_infinity);
    println!("Not a Number: {}", nan);
    
    // NaN is never equal to itself!
    println!("Is NaN == NaN? {}", nan == nan);
    
    // Constants
    println!("\nUseful constants:");
    println!("Pi: {}", std::f64::consts::PI);
    println!("E: {}", std::f64::consts::E);
    
    // Type conversion between integer and float with casting
    let integer = 3;
    let float_from_int = integer as f64;
    let back_to_int = float_from_int as i32;
    
    println!("\nType conversion:");
    println!("Integer: {}", integer);
    println!("Converted to float: {}", float_from_int);
    println!("Back to integer: {}", back_to_int);
    
    // Potential precision loss
    let big_float = 1_000_000.999_f64;
    let as_int = big_float as i32; // This will truncate, not round!
    
    println!("\nPrecision loss:");
    println!("Float: {}", big_float);
    println!("Truncated to int: {}", as_int);
    
    // Challenge: Create a function that calculates the area of a circle given its radius
    // Your code here:
    
    // Challenge: Try comparing two floating-point numbers that should be equal but might
    // have precision issues. Find a better way to compare them.
    // Your code here:
}