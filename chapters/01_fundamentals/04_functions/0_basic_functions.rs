// Basic Functions in Rust
//
// This program demonstrates the fundamentals of defining and calling functions.

fn main() {
    println!("Exploring Basic Functions in Rust!");
    
    // Calling a function
    say_hello();
    
    // Functions can be called multiple times
    say_hello();
    say_hello();
    
    // The code runs sequentially - this prints after the function calls
    println!("Back in the main function!");
    
    // Functions can be defined anywhere in the file
    // but they cannot be nested inside other functions
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Function definition starts with 'fn' keyword followed by the function name
fn say_hello() {
    println!("Hello from the say_hello function!");
}

// This function demonstrates basic structure of a function
fn example_function() {
    // This is the function body
    // It contains all the code that runs when the function is called
    println!("This is an example function!");
    
    // Functions can contain multiple statements
    let x = 5;
    println!("The value of x is: {}", x);
    
    // The function ends at the closing brace
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: Fix this function by adding the fn keyword and correcting the syntax
    say_good_morning() {
        println!("Good morning!");
    }
    
    // TODO: This function should print "Welcome to Rust!" but has multiple issues
    fn print_welcome
        println("Welcome to Rust!")
    }
    
    // TODO: Define a function called 'print_number' that prints the number 42
    // Add your function here
    
    // TODO: Call all three functions in the correct order
    pub fn run_functions() -> Result<(), String> {
        // Call the functions in this order:
        // 1. say_good_morning
        // 2. print_welcome
        // 3. print_number
        
        Ok(())
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    use std::io::{self, Write};
    use std::sync::{Mutex, OnceLock};
    
    // Capture stdout for testing
    fn capture_output<F>(f: F) -> Result<String, String>
    where
        F: FnOnce() -> Result<(), String>,
    {
        static STDOUT_CAPTURE: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
        let stdout_capture = STDOUT_CAPTURE.get_or_init(|| Mutex::new(Vec::new()));
        
        // Lock the buffer for the duration of this function
        let mut buffer = stdout_capture.lock().unwrap();
        buffer.clear(); // Clear previous output
        
        // Temporarily redirect stdout
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.flush(); // Flush any pending output
        
        // Call the provided function, which should write to stdout
        let result = f();
        
        // Get what was written to stdout
        let output = String::from_utf8_lossy(&buffer).to_string();
        
        // Return the output if the function succeeded, otherwise return the error
        result.map(|_| output)
    }
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning Basic Functions challenges...");
        
        match capture_output(challenges::run_functions) {
            Ok(output) => {
                // Check if the output contains all expected strings in order
                let output_lower = output.to_lowercase();
                
                if !output_lower.contains("good morning") {
                    return Err("Missing 'Good morning!' output from say_good_morning function".to_string());
                }
                
                if !output_lower.contains("welcome to rust") {
                    return Err("Missing 'Welcome to Rust!' output from print_welcome function".to_string());
                }
                
                if !output.contains("42") {
                    return Err("Missing '42' output from print_number function".to_string());
                }
                
                // Check the order (roughly)
                let morning_pos = output_lower.find("good morning").unwrap_or(0);
                let welcome_pos = output_lower.find("welcome to rust").unwrap_or(0);
                let number_pos = output.find("42").unwrap_or(0);
                
                if !(morning_pos < welcome_pos && welcome_pos < number_pos) {
                    return Err("Functions were not called in the correct order".to_string());
                }
                
                println!("All functions correctly defined and called in the right order!");
                Ok(())
            },
            Err(e) => Err(format!("run_functions failed: {}", e)),
        }
    }
}