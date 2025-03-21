// This is a comment in Rust

//------------------------------------------------------
// Hello World in Rust
//------------------------------------------------------
//
// This program demonstrates the basic structure of a Rust program
// and how to output text to the console.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch01-02-hello-world.html


// The main function is the entry point of every Rust program
// When you run a Rust program, execution begins in the main function
fn main() {
    //------------------------------------------------------
    // BASIC PRINTING
    //------------------------------------------------------
    // println! is a macro (not a function) that prints text to the console
    // Macros are denoted with an exclamation mark (!)
    // We'll learn more about macros later
    println!("Hello, World!");
    
    //------------------------------------------------------
    // PRINTING WITH VARIABLES
    //------------------------------------------------------
    // You can include variables and expressions in println! using curly braces {}
    let name = "Rustacean";
    println!("Hello, {}!", name);
    
    //------------------------------------------------------
    // MULTIPLE VALUES
    //------------------------------------------------------
    // You can also include multiple values
    let language = "Rust";
    let year = 2025;
    println!("I'm learning {} in {}!", language, year);
    
    //------------------------------------------------------
    // POSITIONAL PARAMETERS
    //------------------------------------------------------
    // You can specify which value goes where using numbers
    println!("In {1}, I'm learning {0}!", language, year);
    
    //------------------------------------------------------
    // NAMED PARAMETERS
    //------------------------------------------------------
    // Named parameters work too
    println!("I'm learning {language} in {year}!");
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: Fix the println! macro invocation
    pub fn broken_print() -> Result<(), String> {
        // Note: it's "println" not "printline"
        printline!("Hello, World!");
        
        Ok(())
    }
    
    // TODO: Make this function print "Hello, Rust!" using the name variable
    pub fn print_with_variable() -> Result<(), String> {
        let name = "Rust";
        println!("Hello!"); // Should print Hello, Rust!
        
        Ok(())
    }
    
    // TODO: Fix this function to print the values in the requested order
    pub fn print_multiple_values() -> Result<(), String> {
        let language = "Rust";
        let year = 2023;
        
        // Should print "Started learning Rust in 2023!"
        println!("Started learning {year} in {language}!");
        
        Ok(())
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    use std::io::{self, Write};
    use std::sync::{Mutex, OnceLock};
    
    // This capture_output function is a simplified way to capture stdout for testing
    // It's not perfect but works for our simple examples
    fn capture_output<F>(f: F) -> Result<String, String>
    where
        F: FnOnce() -> Result<(), String>,
    {
        // Initialize a global mutex to handle concurrent output capture safely
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
        println!("\nRunning Hello World challenges...");
        
        // Challenge 1: Fix the println! macro
        match capture_output(challenges::broken_print) {
            Ok(output) => {
                if !output.contains("Hello, World!") {
                    return Err("broken_print should output 'Hello, World!'".to_string());
                }
                println!("Fixed the broken print statement!");
            },
            Err(e) => return Err(format!("broken_print failed: {}", e)),
        }
        
        // Challenge 2: Print with a variable
        match capture_output(challenges::print_with_variable) {
            Ok(output) => {
                if !output.contains("Hello, Rust!") {
                    return Err("print_with_variable should output 'Hello, Rust!'".to_string());
                }
                println!("Successfully printed with a variable!");
            },
            Err(e) => return Err(format!("print_with_variable failed: {}", e)),
        }
        
        // Challenge 3: Print multiple values in the correct order
        match capture_output(challenges::print_multiple_values) {
            Ok(output) => {
                if !output.contains("Started learning Rust in 2023!") {
                    return Err("print_multiple_values should output 'Started learning Rust in 2023!'".to_string());
                }
                println!("Successfully printed multiple values in the correct order!");
            },
            Err(e) => return Err(format!("print_multiple_values failed: {}", e)),
        }
        
        println!("All Hello World challenges completed successfully!");
        Ok(())
    }
}