// Variables Exercise
// Fix the errors in this code to make it compile.

fn main() {
    // 1. Variables in Rust are immutable by default.
    // Fix this code to make it compile.
    let x = 5;
    x = 6;
    println!("The value of x is: {}", x);

    // 2. You can change a variable's type by shadowing it.
    // Fix this code to make it compile.
    let y = "42";
    let y = y + 8;
    println!("The value of y is: {}", y);

    // 3. Constants require type annotations and their names are usually uppercase.
    // Fix this code to make it compile.
    const max_value = 100;
    println!("The maximum value is: {}", max_value);

    // 4. Fix this code using variable shadowing.
    let z = 10;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z + 3);
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_code_compiles() {
        // If this test runs, it means the code compiled successfully
        assert!(true);
    }
    
    #[test]
    fn test_variable_mutability() {
        // Check if the code properly uses mut for x
        let source = std::fs::read_to_string(file!()).expect("Could not read source file");
        assert!(source.contains("let mut x ="), 
                "You need to use 'let mut x = 5' to make x mutable");
    }
    
    #[test]
    fn test_variable_shadowing() {
        // Check if the code properly shadows y with the correct type conversion
        let source = std::fs::read_to_string(file!()).expect("Could not read source file");
        assert!(source.contains("parse") && source.contains("<i32>"), 
                "You need to use proper type conversion like 'parse::<i32>()' for y");
    }
    
    #[test]
    fn test_constant_declaration() {
        // Check if the code properly declares MAX_VALUE
        let source = std::fs::read_to_string(file!()).expect("Could not read source file");
        assert!(source.contains("const MAX_VALUE:"), 
                "You need to declare MAX_VALUE as a constant with proper syntax");
    }
    
    #[test]
    fn test_variable_shadowing_scope() {
        // Check if the code properly uses shadowing for z
        let source = std::fs::read_to_string(file!()).expect("Could not read source file");
        assert!(source.contains("let z = z * 2"), 
                "You need to use 'let z = z * 2' to properly shadow z in the inner scope");
    }
} 