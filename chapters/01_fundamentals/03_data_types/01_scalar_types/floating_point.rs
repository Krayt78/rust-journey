// Floating-Point Numbers in Rust
//
// Rust has two floating-point types:
// - f32: 32-bit floating point (single precision)
// - f64: 64-bit floating point (double precision)
//
// The default floating-point type is f64.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types

fn main() {
    println!("Exploring Floating-Point Numbers in Rust!");
    
    //------------------------------------------------------
    // BASIC FLOATING-POINT TYPES
    //------------------------------------------------------
    // Basic f64 (default) floating-point numbers
    let x = 2.0; // f64 by default
    println!("x (f64): {}", x);
    
    // Explicit f32
    let y: f32 = 3.0;
    println!("y (f32): {}", y);
    
    //------------------------------------------------------
    // SCIENTIFIC NOTATION
    //------------------------------------------------------
    // Scientific notation
    let large_number = 1.2e4; // 1.2 × 10^4 = 12,000
    let small_number = 1.2e-4; // 1.2 × 10^-4 = 0.00012
    println!("Scientific notation: {} and {}", large_number, small_number);
    
    //------------------------------------------------------
    // ARITHMETIC OPERATIONS
    //------------------------------------------------------
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
    
    //------------------------------------------------------
    // PRECISION LIMITATIONS
    //------------------------------------------------------
    // Precision limitations
    let precision_example = 0.1 + 0.2;
    println!("\nPrecision example:");
    println!("0.1 + 0.2 = {}", precision_example);
    println!("Is 0.1 + 0.2 == 0.3? {}", precision_example == 0.3);
    
    // This happens because floating-point numbers are represented in binary,
    // and some decimal fractions can't be represented exactly in binary.
    
    //------------------------------------------------------
    // SPECIAL VALUES
    //------------------------------------------------------
    // Special floating-point values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN; // Not a Number
    
    println!("\nSpecial values:");
    println!("Infinity: {}", infinity);
    println!("Negative Infinity: {}", neg_infinity);
    println!("NaN: {}", nan);
    
    // NaN comparisons
    println!("Is NaN equal to itself? {}", nan == nan);
    
    //------------------------------------------------------
    // CONSTANTS AND LIMITS
    //------------------------------------------------------
    // Constants and limits
    println!("\nConstants and limits for f64:");
    println!("Smallest positive value: {}", f64::MIN_POSITIVE);
    println!("Largest value: {}", f64::MAX);
    println!("Smallest value: {}", f64::MIN);
    println!("Epsilon (smallest difference): {}", f64::EPSILON);
    
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
    // TODO: These floating-point declarations and calculations have errors
    
    // Error 1: Incorrect type for PI constant
    pub const PI: i32 = 3.14159;
    
    // Error 2: Type mismatch in calculation
    pub fn challenge_float_calculations() -> Result<(), String> {
        let radius: i32 = 5;
        let area = PI * radius * radius;
        
        if (area - 78.5).abs() > 0.1 {
            return Err(format!("Area should be approximately 78.5, got {}", area));
        }
        
        Ok(())
    }
    
    // TODO: This function should calculate the area of a circle but has errors
    pub fn calculate_circle_area(radius: f64) -> f64 {
        // In this function, use the standard library's PI constant
        let pi: f64 = 3.14; // Incorrectly defined PI
        
        // There's also a logical error in the formula
        let area = 2.0 * pi * radius; // This is the formula for circumference, not area
        
        area
    }
    
    // TODO: This function should compare two floats within an epsilon range but has several errors
    pub fn compare_floats(a: f64, b: f64) -> bool {
        // Due to precision issues, we should never directly compare floating point numbers
        a == b // This is incorrect - should use an epsilon comparison
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning floating-point challenges...");
        
        // Challenge 1: Fix floating-point calculations
        if let Err(e) = challenges::challenge_float_calculations() {
            return Err(format!("Challenge 1 failed: {}", e));
        }
        println!("Successfully fixed the floating-point calculations!");
        
        // Challenge 2: Fix the circle area function
        let area = challenges::calculate_circle_area(5.0);
        if (area - 78.54).abs() > 0.01 {
            return Err(format!("Circle area should be approximately 78.54, got {}", area));
        }
        println!("Successfully fixed the circle area function!");
        
        // Challenge 3: Fix the float comparison function
        if !challenges::compare_floats(0.1 + 0.2, 0.3) {
            return Err("compare_floats should return true for 0.1 + 0.2 and 0.3".to_string());
        }
        if challenges::compare_floats(0.1, 0.2) {
            return Err("compare_floats should return false for 0.1 and 0.2".to_string());
        }
        println!("Successfully fixed the float comparison function!");
        
        println!("All floating-point challenges completed successfully!");
        Ok(())
    }
}