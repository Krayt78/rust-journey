fn main() {
    // IMMUTABLE VARIABLES
    // By default, variables in Rust are immutable
    let language = "Rust";
    println!("I'm learning {}", language);
    
    // This would cause a compile error:
    // language = "Rust Programming";
    
    // MUTABLE VARIABLES
    // Use the 'mut' keyword to make a variable mutable
    let mut version = "1.0";
    println!("Starting with version: {}", version);
    
    // Now we can change it
    version = "1.72";
    println!("Updated to version: {}", version);
    
    // SHADOWING
    // We can shadow a variable by using the same name
    let count = 5;
    println!("Original count: {}", count);
    
    // This creates a new variable that shadows the previous one
    let count = count + 5;
    println!("After shadowing: {}", count);
    
    // Shadowing allows changing the type
    let value = "42";
    println!("Value as string: {}", value);
    
    let value = value.parse::<i32>().unwrap();
    println!("Value as integer: {}", value);
    
    // CONSTANTS
    // Constants are always immutable and must have type annotations
    const MAX_USERS: u32 = 100_000;
    println!("Maximum users allowed: {}", MAX_USERS);
    
    // Notice how we use underscores for readability in numbers
    const PI: f64 = 3.141_592_653_589_793;
    println!("Pi: {}", PI);
    
    println!("Variables demonstration complete!");
}