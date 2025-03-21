// Advanced Function Concepts in Rust
//
// This program demonstrates more advanced concepts related to functions in Rust,
// including scope, shadowing in functions, and early returns.

fn main() {
    println!("Exploring Advanced Function Concepts in Rust!");
    
    // Variables defined in main aren't accessible in other functions
    let x = 5;
    println!("In main: x = {}", x);
    
    // Calling a function with its own variable scope
    scope_example();
    // x from main still has its original value
    println!("Back in main: x = {}", x);
    
    // Shadowing in function calls
    let value = 10;
    println!("Before function call: value = {}", value);
    shadowing_example(value);
    println!("After function call: value = {}", value);
    
    // Early returns with conditionals
    let result1 = check_positive(5);
    let result2 = check_positive(-5);
    println!("check_positive(5): {}", result1);
    println!("check_positive(-5): {}", result2);
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Function demonstrating scope
fn scope_example() {
    // This x is different from the x in main
    let x = 10;
    println!("In scope_example: x = {}", x);
    
    // Variables defined in this function aren't accessible outside
    let y = 20;
    println!("In scope_example: y = {}", y);
}

// Function demonstrating parameter shadowing
fn shadowing_example(value: i32) {
    // The parameter 'value' shadows the outer variable with the same name
    println!("In shadowing_example: value = {}", value);
    
    // Local shadowing
    let value = value * 2;
    println!("After shadowing: value = {}", value);
}

// Function with early returns
fn check_positive(num: i32) -> &'static str {
    if num < 0 {
        return "negative";
    }
    
    if num == 0 {
        return "zero";
    }
    
    // This is reached only if the number is positive
    "positive"
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: Fix this function that should find a character in a string
    // and return its position. If not found, return None.
    fn find_char(text: &str, character: char) -> Option<usize> {
        for (i, c) in text.chars().enumerate() {
            if c == character {
                i
            }
        }
        
        None
    }
    
    // TODO: Fix this function to calculate the sum of numbers up to n
    // but skip negative numbers
    fn sum_positive_up_to(n: i32) -> i32 {
        let mut sum = 0;
        
        for i in 0..=n {
            if i < 0 {
                continue;
            }
            sum += i;
        }
        
        return sum;
    }
    
    // TODO: Fix this function that has scoping issues
    fn scoping_problem() -> i32 {
        let result;
        
        {
            let x = 10;
            result = x * 2;
        }
        
        // This should return x * 2
        x
    }
    
    // TODO: Fix this function that should calculate the absolute difference
    // between two numbers
    fn absolute_difference(a: i32, b: i32) -> i32 {
        let difference = a - b;
        
        if difference < 0 {
            return -difference
        }
        
        difference
    }
    
    // TODO: Fix and call the functions above
    pub fn run_functions() -> Result<(), String> {
        // Test find_char
        let position = find_char("hello", 'e');
        if position != Some(1) {
            return Err(format!("find_char(\"hello\", 'e') returned {:?}, expected Some(1)", position));
        }
        
        let not_found = find_char("hello", 'z');
        if not_found != None {
            return Err(format!("find_char(\"hello\", 'z') returned {:?}, expected None", not_found));
        }
        
        // Test sum_positive_up_to
        let sum = sum_positive_up_to(5);
        if sum != 15 { // 0 + 1 + 2 + 3 + 4 + 5 = 15
            return Err(format!("sum_positive_up_to(5) returned {}, expected 15", sum));
        }
        
        // Test scoping_problem
        let scoping_result = scoping_problem();
        if scoping_result != 20 {
            return Err(format!("scoping_problem() returned {}, expected 20", scoping_result));
        }
        
        // Test absolute_difference
        let diff1 = absolute_difference(5, 10);
        if diff1 != 5 {
            return Err(format!("absolute_difference(5, 10) returned {}, expected 5", diff1));
        }
        
        let diff2 = absolute_difference(10, 5);
        if diff2 != 5 {
            return Err(format!("absolute_difference(10, 5) returned {}, expected 5", diff2));
        }
        
        println!("All advanced function challenges completed successfully!");
        Ok(())
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Advanced Function Concepts challenges...");
        
        challenges::run_functions()
    }
}