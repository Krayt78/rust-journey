// Function Parameters in Rust
//
// This program demonstrates how to define functions with parameters
// and pass arguments to them.

fn main() {
    println!("Exploring Function Parameters in Rust!");
    
    // Calling a function with a single parameter
    greet("Rustacean");
    
    // Calling with a different argument
    greet("World");
    
    // Functions with multiple parameters
    add_and_print(5, 7);
    
    // Arguments can be expressions
    let x = 10;
    let y = 20;
    add_and_print(x + 5, y);
    
    // Note: Rust is strongly typed, so parameters must have type annotations
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// Function with a single parameter
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with multiple parameters
fn add_and_print(a: i32, b: i32) {
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: Fix this function by adding the missing parameter type
    fn multiply(a: i32, b) {
        println!("{} * {} = {}", a, b, a * b);
    }
    
    // TODO: Fix this function to take two parameters: name and age
    fn describe_person(name) {
        println!("{} is {} years old", name, age);
    }
    
    // TODO: Fix this function that should print a greeting message
    // from one person to another
    fn send_greeting(from, to: &str) {
        println!("From {}: Hello, {}!", to, from);
    }
    
    // TODO: Fix and call the functions above
    pub fn run_functions() -> Result<(), String> {
        // Call multiply with 6 and 7
        multiply(6);
        
        // Call describe_person with "Alice" and 30
        describe_person("Alice");
        
        // Call send_greeting with "Bob" and "Charlie"
        send_greeting("Bob");
        
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
        println!("\nRunning Function Parameters challenges...");
        
        match capture_output(challenges::run_functions) {
            Ok(output) => {
                // Check for multiply output
                if !output.contains("6 * 7 = 42") {
                    return Err("Missing or incorrect output from multiply function. Expected '6 * 7 = 42'".to_string());
                }
                
                // Check for describe_person output
                if !output.contains("Alice is 30 years old") {
                    return Err("Missing or incorrect output from describe_person function. Expected 'Alice is 30 years old'".to_string());
                }
                
                // Check for send_greeting output
                if !output.contains("From Bob: Hello, Charlie!") {
                    return Err("Missing or incorrect output from send_greeting function. Expected 'From Bob: Hello, Charlie!'".to_string());
                }
                
                println!("All functions with parameters work correctly!");
                Ok(())
            },
            Err(e) => Err(format!("run_functions failed: {}", e)),
        }
    }
}