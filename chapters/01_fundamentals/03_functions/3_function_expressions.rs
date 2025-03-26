// Function Expressions in Rust
//
// This program demonstrates the difference between statements and expressions
// in Rust functions, and how they affect return values.

fn main() {
    println!("Exploring Expressions in Rust Functions!");
    
    // In Rust, the last expression in a function is returned
    let sum = add(5, 7);
    println!("Sum: {}", sum);
    
    // Using the if expression as a return value
    let result = check_number(15);
    println!("Result of check_number(15): {}", result);
    
    // Using a block expression as a return value
    let computed = compute_value(10);
    println!("Computed value: {}", computed);
    
    // Expression with conditionals
    let status_message = get_status_message(200);
    println!("Status 200: {}", status_message);
    
    let error_message = get_status_message(404);
    println!("Status 404: {}", error_message);
}

// Function that returns the sum of two numbers
fn add(a: i32, b: i32) -> i32 {
    // This is an expression (no semicolon)
    a + b
}

// Function that returns a string based on a condition
fn check_number(x: i32) -> &'static str {
    // if-else is an expression in Rust that returns a value
    if x > 10 {
        "greater than ten"  // No semicolon
    } else {
        "less than or equal to ten"  // No semicolon
    }
}

// Function that uses a block expression
fn compute_value(x: i32) -> i32 {
    // This entire block is an expression
    {
        let a = x * 2;
        let b = a + 10;
        // The last expression in the block is returned
        b - 5
    }
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: Fix this function to return the absolute value of a number
    // using an if expression
    fn absolute_value(n: i32) -> i32 {
        if n < 0 {
            return -n;
        } else {
            return n;
        }
    }
    
    // TODO: Fix this function to return a message based on the score
    // "Excellent" for scores 90-100
    // "Good" for scores 70-89
    // "Passing" for scores 50-69
    // "Failing" for scores 0-49
    fn grade_message(score: i32) -> &'static str {
        // This implementation is missing something
        if score >= 90 && score <= 100 {
            "Excellent";
        } else if score >= 70 && score <= 89 {
            "Good";
        } else if score >= 50 && score <= 69 {
            "Passing";
        } else {
            "Failing";
        }
    }
    
    // TODO: Fix this function to calculate factorial using a block expression
    fn factorial(n: u32) -> u32 {
        // This implementation has a syntax error
        {
            let mut result = 1;
            
            for i in 1..=n {
                result *= i;
            }
            
            result;
        }
    }
    
    // TODO: Fix and call the functions above
    pub fn run_functions() -> Result<(), String> {
        // Test absolute_value
        let abs_tests = [
            (5, 5),
            (-5, 5),
            (0, 0),
            (-10, 10)
        ];
        
        for (input, expected) in abs_tests {
            let result = absolute_value(input);
            if result != expected {
                return Err(format!("absolute_value({}) returned {}, expected {}", 
                                   input, result, expected));
            }
        }
        
        // Test grade_message
        let grade_tests = [
            (95, "Excellent"),
            (85, "Good"),
            (65, "Passing"),
            (45, "Failing")
        ];
        
        for (score, expected) in grade_tests {
            let message = grade_message(score);
            if message != expected {
                return Err(format!("grade_message({}) returned '{}', expected '{}'", 
                                  score, message, expected));
            }
        }
        
        // Test factorial
        let fact_tests = [
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 6),
            (4, 24),
            (5, 120)
        ];
        
        for (n, expected) in fact_tests {
            let result = factorial(n);
            if result != expected {
                return Err(format!("factorial({}) returned {}, expected {}", 
                                  n, result, expected));
            }
        }
        
        println!("All function expression challenges completed successfully!");
        Ok(())
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Function Expressions challenges...");
        
        challenges::run_functions()
    }
}