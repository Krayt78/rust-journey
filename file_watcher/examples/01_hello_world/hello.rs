// Hello World Exercise
// In this exercise, you need to complete the main function
// to print "Hello, World!" to the console.

fn main() {
    // TODO: Print "Hello, World!" to the console
    //println!("Hello, World!");
    
    
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_code_compiles() {
        // If this test runs, it means the code compiled successfully
        assert!(false);
    }
    
    #[test]
    fn test_hello_world_output() {
        // Check if the code contains the proper println! statement
        let source = std::fs::read_to_string(file!()).expect("Could not read source file");
        assert!(source.contains("println!(\"Hello, World!\")"), 
                "You need to print 'Hello, World!' using the println! macro");
    }
} 