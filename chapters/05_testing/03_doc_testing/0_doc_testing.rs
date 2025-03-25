// Documentation Testing in Rust
//
// This file demonstrates how to write effective documentation tests in Rust
// and how they help ensure your code examples work as expected.

fn main() {
    println!("Documentation Testing in Rust");
    println!("Run the tests with: cargo test --doc");
    
    // Try out some of the functions with documentation tests
    let rect = Rectangle::new(5, 10);
    println!("Area of rectangle: {}", rect.area());
    
    let result = vector_sum(&[1, 2, 3, 4, 5]);
    println!("Sum of vector: {}", result);
    
    let palindrome = is_palindrome("racecar");
    println!("Is 'racecar' a palindrome? {}", palindrome);
    
    println!("\nComplete the exercises at the end of this file.");
}

//------------------------------------------------------
// BASIC DOCUMENTATION TESTS
//------------------------------------------------------

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = doc_testing::add(2, 3);
/// assert_eq!(result, 5);
///
/// let negative = doc_testing::add(-1, -2);
/// assert_eq!(negative, -3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calculates the sum of a vector of integers.
///
/// Returns 0 if the vector is empty.
///
/// # Examples
///
/// ```
/// let empty = doc_testing::vector_sum(&[]);
/// assert_eq!(empty, 0);
///
/// let sum = doc_testing::vector_sum(&[1, 2, 3, 4, 5]);
/// assert_eq!(sum, 15);
/// ```
pub fn vector_sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

//------------------------------------------------------
// DOCUMENTING STRUCTS AND METHODS
//------------------------------------------------------

/// A rectangle with width and height.
///
/// # Examples
///
/// ```
/// let rect = doc_testing::Rectangle::new(10, 20);
/// assert_eq!(rect.width(), 10);
/// assert_eq!(rect.height(), 20);
/// ```
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Creates a new rectangle with the given width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = doc_testing::Rectangle::new(5, 10);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    /// Returns the width of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = doc_testing::Rectangle::new(5, 10);
    /// assert_eq!(rect.width(), 5);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Returns the height of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = doc_testing::Rectangle::new(5, 10);
    /// assert_eq!(rect.height(), 10);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Calculates the area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = doc_testing::Rectangle::new(5, 10);
    /// assert_eq!(rect.area(), 50);
    /// ```
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    /// Calculates the perimeter of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = doc_testing::Rectangle::new(5, 10);
    /// assert_eq!(rect.perimeter(), 30); // 2*(5+10) = 30
    /// ```
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

//------------------------------------------------------
// HIDING CODE IN DOCUMENTATION
//------------------------------------------------------

/// A complex number with real and imaginary parts.
///
/// # Examples
///
/// ```
/// # // This import is hidden in the documentation but needed for the test
/// # use doc_testing::Complex;
/// let c = Complex::new(3.0, 4.0);
/// assert_eq!(c.magnitude(), 5.0);
/// ```
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    /// Creates a new complex number.
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
    
    /// Calculates the magnitude (absolute value) of the complex number.
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

//------------------------------------------------------
// DOCUMENTING PANICS
//------------------------------------------------------

/// Divides two numbers.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// # Examples
///
/// ```
/// let result = doc_testing::divide(10.0, 2.0);
/// assert_eq!(result, 5.0);
/// ```
///
/// ```should_panic
/// // This will panic
/// doc_testing::divide(10.0, 0.0);
/// ```
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Division by zero");
    }
    a / b
}

//------------------------------------------------------
// DOCUMENTING COMPILE FAILURES
//------------------------------------------------------

/// A wrapper type that only accepts positive integers.
///
/// # Examples
///
/// ```
/// let positive = doc_testing::Positive::new(42);
/// assert_eq!(positive.value(), 42);
/// ```
///
/// ```compile_fail
/// // This won't compile because -10 is negative
/// let negative = doc_testing::Positive::new(-10);
/// ```
pub struct Positive {
    value: u32,
}

impl Positive {
    /// Creates a new Positive value.
    ///
    /// Accepts only non-negative integers.
    pub fn new(value: i32) -> Self {
        if value < 0 {
            panic!("Positive can only be created with non-negative values");
        }
        Positive { value: value as u32 }
    }
    
    /// Returns the value.
    pub fn value(&self) -> u32 {
        self.value
    }
}

//------------------------------------------------------
// MODULE DOCUMENTATION
//------------------------------------------------------

/// A simple string utility module.
///
/// This module provides functions for working with strings.
///
/// # Examples
///
/// Check if a string is a palindrome:
///
/// ```
/// assert!(doc_testing::is_palindrome("racecar"));
/// assert!(!doc_testing::is_palindrome("hello"));
/// ```
pub mod string_utils {
    /// Checks if a string is empty or contains only whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use doc_testing::string_utils::is_blank;
    ///
    /// assert!(is_blank(""));
    /// assert!(is_blank("   "));
    /// assert!(!is_blank("hello"));
    /// ```
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }
    
    /// Reverses a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use doc_testing::string_utils::reverse;
    ///
    /// assert_eq!(reverse("hello"), "olleh");
    /// assert_eq!(reverse("rust"), "tsur");
    /// ```
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    
    /// Counts the occurrences of a character in a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use doc_testing::string_utils::count_char;
    ///
    /// assert_eq!(count_char("hello", 'l'), 2);
    /// assert_eq!(count_char("rust", 'z'), 0);
    /// ```
    pub fn count_char(s: &str, c: char) -> usize {
        s.chars().filter(|&ch| ch == c).count()
    }
}

/// Checks if a string is a palindrome (reads the same forwards and backwards).
///
/// A palindrome is a word, phrase, number, or other sequence of characters
/// that reads the same forward and backward, ignoring spaces, punctuation,
/// and capitalization.
///
/// # Examples
///
/// Basic palindromes:
/// ```
/// assert!(doc_testing::is_palindrome("racecar"));
/// assert!(doc_testing::is_palindrome("level"));
/// assert!(!doc_testing::is_palindrome("hello"));
/// ```
///
/// Ignoring spaces and capitalization:
/// ```
/// assert!(doc_testing::is_palindrome("A man a plan a canal Panama"));
/// assert!(doc_testing::is_palindrome("Madam Im Adam"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    // Remove spaces and convert to lowercase
    let s: String = s.chars()
        .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    // Check if the string is the same forwards and backwards
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Add Documentation with Examples
//
// Add proper documentation and examples to the following function.
// Include at least one example that demonstrates its usage.

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Exercise 2: Document a Function with Panics
//
// Document the following function and include examples that show
// both normal usage and the panic case.

pub fn get_element(v: &[i32], index: usize) -> i32 {
    if index >= v.len() {
        panic!("Index out of bounds");
    }
    v[index]
}

// Exercise 3: Document a Struct with Methods
//
// Add appropriate documentation to the Stack struct and its methods.
// Include examples demonstrating how to use the Stack.

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// Exercise 4: Document a Module
//
// Create documentation for the following module. Include a module-level
// documentation comment and examples for each function.

pub mod math {
    pub fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    
    pub fn lcm(a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            a * b / gcd(a, b)
        }
    }
    
    pub fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        
        true
    }
} 