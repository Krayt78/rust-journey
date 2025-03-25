// Unit Testing in Rust
//
// This file demonstrates how to write and organize unit tests in Rust,
// covering basic tests, test-driven development, and testing for failures.

fn main() {
    println!("Unit Testing in Rust");
    println!("Run the tests with: cargo test");
    
    // Try out some of the functions we'll be testing
    let rect = Rectangle { width: 5, height: 10 };
    println!("Area of rectangle: {}", rect.area());
    println!("Perimeter of rectangle: {}", rect.perimeter());
    
    println!("Factorial of 5: {}", factorial(5));
    
    let result = divide(10, 2);
    println!("10 / 2 = {}", result);
    
    // This would panic:
    // let result = divide(10, 0);
    
    let temp_f = 77.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F = {}°C", temp_f, temp_c);
    
    println!("\nComplete the exercises at the end of this file.");
}

//------------------------------------------------------
// BASIC UNIT TESTING
//------------------------------------------------------

// A simple function to add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// A function to calculate the factorial of a number
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Unit tests for the add and factorial functions
#[cfg(test)]
mod basic_tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}

//------------------------------------------------------
// TESTING FAILURES (SHOULD_PANIC)
//------------------------------------------------------

// A function that divides two numbers, panicking if the divisor is zero
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

// Tests for the divide function, including expected panic
#[cfg(test)]
mod failure_tests {
    use super::*;
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
        assert_eq!(divide(0, 5), 0);
        assert_eq!(divide(-10, 2), -5);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
}

//------------------------------------------------------
// TESTING STRUCTS AND METHODS
//------------------------------------------------------

// A Rectangle struct with methods for area and perimeter
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Tests for the Rectangle struct and its methods
#[cfg(test)]
mod rectangle_tests {
    use super::*;
    
    #[test]
    fn test_area() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.area(), 50);
        
        let square = Rectangle::new(5, 5);
        assert_eq!(square.area(), 25);
    }
    
    #[test]
    fn test_perimeter() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.perimeter(), 30);
        
        let square = Rectangle::new(5, 5);
        assert_eq!(square.perimeter(), 20);
    }
    
    #[test]
    fn test_is_square() {
        let rect = Rectangle::new(5, 10);
        assert!(!rect.is_square());
        
        let square = Rectangle::new(5, 5);
        assert!(square.is_square());
    }
}

//------------------------------------------------------
// TEST ORGANIZATION
//------------------------------------------------------

// Function to convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Organized tests for temperature conversion functions
#[cfg(test)]
mod temperature_tests {
    use super::*;
    
    // Test fixture: common setup code
    fn setup() -> Vec<(f64, f64)> {
        // Pairs of (Fahrenheit, Celsius) values
        vec![
            (32.0, 0.0),     // Freezing point
            (212.0, 100.0),  // Boiling point
            (98.6, 37.0),    // Body temperature (rounded)
            (-40.0, -40.0),  // Interesting point: F = C
        ]
    }
    
    #[test]
    fn test_f_to_c() {
        let test_cases = setup();
        
        for (f, c) in test_cases {
            // Use a delta for floating-point comparisons
            assert!((fahrenheit_to_celsius(f) - c).abs() < 0.1, 
                    "Failed conversion: {}°F should be approx {}°C", f, c);
        }
    }
    
    #[test]
    fn test_c_to_f() {
        let test_cases = setup();
        
        for (f, c) in test_cases {
            // Use a delta for floating-point comparisons
            assert!((celsius_to_fahrenheit(c) - f).abs() < 0.1,
                    "Failed conversion: {}°C should be approx {}°F", c, f);
        }
    }
    
    #[test]
    fn test_round_trip() {
        // Property-based test: Converting from F to C and back should
        // approximately return the original value
        let temperatures = vec![0.0, 32.0, 68.0, 100.0, -10.0];
        
        for temp in temperatures {
            let round_trip = celsius_to_fahrenheit(fahrenheit_to_celsius(temp));
            assert!((round_trip - temp).abs() < 0.0001,
                    "Round trip conversion failed for {}°F", temp);
        }
    }
}

//------------------------------------------------------
// PRIVATE FUNCTION TESTING
//------------------------------------------------------

// A module with a private function
mod calculator {
    // Private function - not exposed outside this module
    fn square(x: i32) -> i32 {
        x * x
    }
    
    // Public function that uses the private function
    pub fn sum_of_squares(a: i32, b: i32) -> i32 {
        square(a) + square(b)
    }
    
    // Tests within the same module can access private functions
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_square() {
            assert_eq!(square(2), 4);
            assert_eq!(square(-3), 9);
            assert_eq!(square(0), 0);
        }
        
        #[test]
        fn test_sum_of_squares() {
            assert_eq!(sum_of_squares(2, 3), 13); // 2² + 3² = 4 + 9 = 13
            assert_eq!(sum_of_squares(0, 0), 0);
            assert_eq!(sum_of_squares(-1, -1), 2); // (-1)² + (-1)² = 1 + 1 = 2
        }
    }
}

//------------------------------------------------------
// TEST DRIVEN DEVELOPMENT EXAMPLE
//------------------------------------------------------

// Let's implement a simple string utility library using TDD.
// We'll start with the tests, then implement the functions.

pub mod string_utils {
    // Function to remove all whitespace from a string
    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
    
    // Function to check if a string is a palindrome
    pub fn is_palindrome(s: &str) -> bool {
        let s = remove_whitespace(&s.to_lowercase());
        let reversed: String = s.chars().rev().collect();
        s == reversed
    }
    
    // Function to count the frequency of a character in a string
    pub fn char_frequency(s: &str, c: char) -> usize {
        s.chars().filter(|&ch| ch == c).count()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_remove_whitespace() {
            assert_eq!(remove_whitespace("hello world"), "helloworld");
            assert_eq!(remove_whitespace("   spaces   "), "spaces");
            assert_eq!(remove_whitespace("\ttabs\nand\nnewlines"), "tabsandnewlines");
        }
        
        #[test]
        fn test_is_palindrome() {
            assert!(is_palindrome("racecar"));
            assert!(is_palindrome("A man a plan a canal Panama"));
            assert!(is_palindrome("Madam, I'm Adam"));
            assert!(!is_palindrome("hello"));
            assert!(!is_palindrome("This is not a palindrome"));
        }
        
        #[test]
        fn test_char_frequency() {
            assert_eq!(char_frequency("hello", 'l'), 2);
            assert_eq!(char_frequency("Mississippi", 's'), 4);
            assert_eq!(char_frequency("counting", 'z'), 0);
        }
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Implement and Test a Vector Sum Function
//
// Implement a function `sum_vector` that takes a vector of integers
// and returns their sum. Write tests for empty vectors, vectors with one
// element, and vectors with multiple elements.

// TODO: Implement sum_vector
pub fn sum_vector(v: &[i32]) -> i32 {
    // Your implementation here
    unimplemented!()
}

// Exercise 2: Roman Numeral Converter
//
// Implement a function `to_roman` that converts an integer (1-3999) to
// a Roman numeral string. Write tests for various cases, including
// edge cases. If the input is outside the valid range, return None.

// TODO: Implement to_roman
pub fn to_roman(num: u16) -> Option<String> {
    // Your implementation here
    unimplemented!()
}

// Exercise 3: Implement a Stack with Tests
//
// Implement a generic Stack<T> with push, pop, peek, and is_empty methods.
// Write tests to verify the correctness of each method, including edge cases
// like popping from an empty stack.

// TODO: Implement Stack<T>
// pub struct Stack<T> { ... }
// impl<T> Stack<T> { ... }

// Exercise 4: Test-Driven Password Validator
//
// Using TDD, implement a password validator that checks if a password:
// 1. Is at least 8 characters long
// 2. Contains at least one uppercase letter
// 3. Contains at least one lowercase letter
// 4. Contains at least one digit
// 5. Contains at least one special character (!@#$%^&*()_+)
//
// Start by writing tests for each requirement, then implement the validation function.

// TODO: Implement validate_password
pub fn validate_password(password: &str) -> bool {
    // Your implementation here
    unimplemented!()
}

//------------------------------------------------------
// TESTS FOR EXERCISES
//------------------------------------------------------

#[cfg(test)]
mod exercise_tests {
    use super::*;
    
    // Tests for Exercise 1
    #[test]
    fn test_sum_vector() {
        assert_eq!(sum_vector(&[]), 0);
        assert_eq!(sum_vector(&[5]), 5);
        assert_eq!(sum_vector(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_vector(&[-1, 1]), 0);
        assert_eq!(sum_vector(&[-10, -5, -1]), -16);
    }
    
    // Tests for Exercise 2
    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(1), Some("I".to_string()));
        assert_eq!(to_roman(4), Some("IV".to_string()));
        assert_eq!(to_roman(9), Some("IX".to_string()));
        assert_eq!(to_roman(14), Some("XIV".to_string()));
        assert_eq!(to_roman(42), Some("XLII".to_string()));
        assert_eq!(to_roman(99), Some("XCIX".to_string()));
        assert_eq!(to_roman(2023), Some("MMXXIII".to_string()));
        assert_eq!(to_roman(3999), Some("MMMCMXCIX".to_string()));
        
        // Edge cases
        assert_eq!(to_roman(0), None);
        assert_eq!(to_roman(4000), None);
    }
    
    // Tests for Exercise 3
    // #[test]
    // fn test_stack() {
    //     let mut stack: Stack<i32> = Stack::new();
    //     
    //     assert!(stack.is_empty());
    //     assert_eq!(stack.peek(), None);
    //     assert_eq!(stack.pop(), None);
    //     
    //     stack.push(1);
    //     assert!(!stack.is_empty());
    //     assert_eq!(stack.peek(), Some(&1));
    //     
    //     stack.push(2);
    //     stack.push(3);
    //     assert_eq!(stack.peek(), Some(&3));
    //     
    //     assert_eq!(stack.pop(), Some(3));
    //     assert_eq!(stack.pop(), Some(2));
    //     assert_eq!(stack.pop(), Some(1));
    //     assert_eq!(stack.pop(), None);
    //     assert!(stack.is_empty());
    // }
    
    // Tests for Exercise 4
    #[test]
    fn test_password_length() {
        assert!(!validate_password("short"));
        assert!(validate_password("LongEnough123!"));
    }
    
    #[test]
    fn test_password_uppercase() {
        assert!(!validate_password("nouppercase123!"));
        assert!(validate_password("Uppercase123!"));
    }
    
    #[test]
    fn test_password_lowercase() {
        assert!(!validate_password("NOLOWERCASE123!"));
        assert!(validate_password("Lowercase123!"));
    }
    
    #[test]
    fn test_password_digit() {
        assert!(!validate_password("NoDigits!"));
        assert!(validate_password("WithDigit1!"));
    }
    
    #[test]
    fn test_password_special_char() {
        assert!(!validate_password("NoSpecialChar123"));
        assert!(validate_password("WithSpecial123!"));
    }
    
    #[test]
    fn test_password_all_requirements() {
        assert!(validate_password("ValidP@ssw0rd"));
        assert!(validate_password("An0ther!Valid1"));
        assert!(!validate_password("invalid"));
    }
} 