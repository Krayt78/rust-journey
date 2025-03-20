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
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// CHALLENGES: TODO: Fix the broken code in this module
mod challenges {
    // TODO: These floating-point declarations and calculations have errors
    
    // Error 1: Type mismatch between f32 and f64
    let float32: f32 = 10.5;
    let float64 = 20.7;
    let sum = float32 + float64;
    
    // Error 2: Dividing by zero
    let zero = 0.0;
    let result = 5.0 / zero;
    println!("Division result: {}", result);
    
    // Error 3: Incorrect scientific notation
    let scientific = 1.5e;
    
    // Error 4: Wrong modulo operation
    let remainder = 10.5 %% 3.0;
    
    // TODO: This function should calculate the area of a circle but has errors
    pub fn calculate_circle_area(radius: f64) -> f64 {
        // The area of a circle is π * r²
        // This implementation has errors in the formula
        let pi = 3.14; // Not using the standard constant
        pi + radius * radius // Wrong operation
    }
    
    // TODO: This function should compare two floats within an epsilon range
    // but has several errors
    pub fn compare_floats(a: f64, b: f64, epsilon: f64) -> bool {
        // This should return true if the absolute difference between a and b
        // is less than or equal to epsilon
        a - b < epsilon // Missing absolute value, wrong comparison
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning floating-point challenges...");
        
        // Challenge 1: Fix floating-point declarations and calculations
        challenge_float_calculations()?;
        
        // Challenge 2: Fix the circle area calculation function
        let area = challenges::calculate_circle_area(2.0);
        let expected = std::f64::consts::PI * 4.0; // π * r²
        
        if (area - expected).abs() > 0.0001 {
            return Err(format!("Circle area calculation wrong. Expected: {}, Got: {}", expected, area));
        }
        println!("Circle area calculation correct!");
        
        // Challenge 3: Fix the float comparison function
        // Should return true if a and b are within epsilon of each other
        if !challenges::compare_floats(0.1 + 0.2, 0.3, 0.0001) {
            return Err("compare_floats should return true for 0.1 + 0.2 and 0.3".to_string());
        }
        println!("Float comparison function works correctly!");
        
        println!("All floating-point challenges completed successfully!");
        Ok(())
    }
    
    fn challenge_float_calculations() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Float calculations challenge completed!");
        Ok(())
    }
}