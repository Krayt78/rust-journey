// Function Expressions in Rust
//
// This program demonstrates the difference between statements and expressions
// in Rust functions, and how they affect return values.

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in these functions to make them compile
//------------------------------------------------------

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

fn main() {
} 