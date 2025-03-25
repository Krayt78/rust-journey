// Strings in Rust
//
// This file demonstrates how to work with Rust's string types: String and &str.
// Rust strings are UTF-8 encoded, which has implications for how they're handled.

fn main() {
    println!("Understanding Strings in Rust!");
    
    //------------------------------------------------------
    // STRING TYPES: String vs &str
    //------------------------------------------------------
    println!("\n=== String Types ===");
    
    // String literal (&str) - fixed-size, immutable string slice
    let string_literal = "Hello, world!";
    println!("String literal (&str): {}", string_literal);
    
    // String - growable, heap-allocated string
    let owned_string = String::from("Hello, world!");
    println!("Owned string (String): {}", owned_string);
    
    // Converting between String and &str
    let s = String::from("Hello");
    let s_slice: &str = &s; // Convert String to &str by referencing
    println!("String as &str: {}", s_slice);
    
    let literal = "World";
    let s_owned = literal.to_string(); // Convert &str to String
    let s_owned2 = String::from(literal); // Another way to convert
    println!("&str as String: {}", s_owned);
    println!("&str as String (2nd way): {}", s_owned2);
    
    //------------------------------------------------------
    // CREATING STRINGS
    //------------------------------------------------------
    println!("\n=== Creating Strings ===");
    
    // Create an empty String
    let mut empty_string = String::new();
    println!("Empty string: '{}'", empty_string);
    println!("Is it empty? {}", empty_string.is_empty());
    
    // Create a String with initial text
    let hello = String::from("Hello");
    println!("Initial string: {}", hello);
    
    // Create a String with capacity
    let mut with_capacity = String::with_capacity(20);
    println!(
        "String with capacity: '{}', Capacity: {}, Length: {}",
        with_capacity,
        with_capacity.capacity(),
        with_capacity.len()
    );
    
    // Adding to the string with capacity
    with_capacity.push_str("Hello, world!");
    println!(
        "After adding text: '{}', Capacity: {}, Length: {}",
        with_capacity,
        with_capacity.capacity(),
        with_capacity.len()
    );
    
    //------------------------------------------------------
    // UPDATING STRINGS
    //------------------------------------------------------
    println!("\n=== Updating Strings ===");
    
    // Using push_str to append a string slice
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("After push_str: {}", s);
    
    // Using push to append a single character
    s.push('!');
    println!("After push: {}", s);
    
    // String concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + takes ownership of s1 and borrows s2
    let s3 = s1 + &s2; // s1 is moved and can no longer be used
    println!("Concatenated: {}", s3);
    // println!("s1: {}", s1); // This would cause an error
    
    // Concatenating multiple strings with +
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let game = s1 + "-" + &s2 + "-" + &s3; // s1 is moved
    println!("Multiple concatenation: {}", game);
    
    // Using format! macro for concatenation
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let message = format!("{}, {}!", s1, s2); // Doesn't take ownership
    println!("format! result: {}", message);
    println!("s1 still valid: {}", s1); // s1 is still valid
    
    //------------------------------------------------------
    // INDEXING AND SLICING
    //------------------------------------------------------
    println!("\n=== Indexing and Slicing ===");
    
    let hello = String::from("Hello");
    
    // Slicing works on byte indices
    let h = &hello[0..1];
    let e = &hello[1..2];
    println!("First two chars by slice: {} {}", h, e);
    
    // Slicing with a range
    let slice = &hello[0..3];
    println!("Slice of first 3 bytes: {}", slice);
    
    // Be careful with Unicode!
    let namaste = String::from("नमस्ते");
    println!("Unicode string: {}", namaste);
    
    // This would panic because we're slicing in the middle of a char:
    // let slice = &namaste[0..1]; // ERROR: not a char boundary
    
    //------------------------------------------------------
    // ITERATING OVER STRINGS
    //------------------------------------------------------
    println!("\n=== Iterating Over Strings ===");
    
    let hello = String::from("Hello");
    
    // Iterate over characters
    println!("Characters in 'Hello':");
    for c in hello.chars() {
        println!("  {}", c);
    }
    
    // Iterate over bytes
    println!("Bytes in 'Hello':");
    for b in hello.bytes() {
        println!("  {}", b);
    }
    
    // Unicode example
    let namaste = String::from("नमस्ते"); // Namaste in Hindi
    
    println!("Characters in 'नमस्ते':");
    for c in namaste.chars() {
        println!("  {}", c);
    }
    
    println!("Bytes in 'नमस्ते':");
    for b in namaste.bytes() {
        println!("  {}", b);
    }
    
    println!(
        "'नमस्ते' - Length in bytes: {}, Character count: {}",
        namaste.len(),
        namaste.chars().count()
    );
    
    //------------------------------------------------------
    // STRING METHODS
    //------------------------------------------------------
    println!("\n=== String Methods ===");
    
    let mut message = String::from("  Hello, Rust World!  ");
    println!("Original: '{}'", message);
    
    // Trimming whitespace
    let trimmed = message.trim();
    println!("Trimmed: '{}'", trimmed);
    
    // Replacing
    let replaced = message.replace("Rust", "Amazing Rust");
    println!("Replaced: '{}'", replaced);
    
    // Case conversion
    let uppercase = message.to_uppercase();
    println!("Uppercase: '{}'", uppercase);
    
    let lowercase = message.to_lowercase();
    println!("Lowercase: '{}'", lowercase);
    
    // Contains, starts_with, ends_with
    println!("Contains 'Rust': {}", message.contains("Rust"));
    println!("Starts with ' Hello': {}", message.starts_with(" Hello"));
    println!("Ends with '!  ': {}", message.ends_with("!  "));
    
    // Splitting
    println!("Split by comma:");
    for part in message.split(',') {
        println!("  '{}'", part);
    }
    
    // Joining
    let parts = ["Hello", "Rust", "Programming"];
    let joined = parts.join(", ");
    println!("Joined: '{}'", joined);
    
    //------------------------------------------------------
    // STRING ALLOCATION AND PERFORMANCE
    //------------------------------------------------------
    println!("\n=== String Allocation and Performance ===");
    
    let mut s = String::with_capacity(25);
    println!(
        "Initial: Length = {}, Capacity = {}",
        s.len(),
        s.capacity()
    );
    
    // Add content up to capacity
    s.push_str("Hello, this fits in capacity");
    println!(
        "Added content: Length = {}, Capacity = {}",
        s.len(),
        s.capacity()
    );
    
    // Exceed the capacity - will reallocate
    s.push_str(". This will exceed capacity.");
    println!(
        "Exceeded capacity: Length = {}, Capacity = {}",
        s.len(),
        s.capacity()
    );
    
    // Shrink to fit
    s.shrink_to_fit();
    println!(
        "After shrink_to_fit: Length = {}, Capacity = {}",
        s.len(),
        s.capacity()
    );
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: String Concatenation
//
// Implement the function that takes a vector of String values
// and returns a single String with each input separated by a space.

fn join_strings(strings: Vec<String>) -> String {
    // TODO: Implement this function.
    // Join all the strings with a space between them.
    // Return the resulting string.
    
    String::new() // Replace this placeholder
}

// Challenge 2: String Transformation
//
// Implement a function that takes a sentence and:
// 1. Capitalizes the first letter of each word
// 2. Replaces any occurrences of "rust" (case insensitive) with "Rust"
// 3. Removes any extra whitespace

fn transform_sentence(sentence: &str) -> String {
    // TODO: Implement this function.
    // Capitalize the first letter of each word
    // Replace any occurrence of "rust" with "Rust" (case insensitive)
    // Remove any extra whitespace
    // Return the transformed string.
    
    String::new() // Replace this placeholder
}

// Challenge 3: Character Count
//
// Implement a function that returns a HashMap with each character
// as a key and the number of occurrences as the value.
// Treat uppercase and lowercase letters as the same character.

use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, usize> {
    // TODO: Implement this function.
    // Count the occurrences of each character (case insensitive)
    // Return a HashMap with character -> count
    
    HashMap::new() // Replace this placeholder
}

// Challenge 4: Palindrome Checker
//
// Implement a function that checks if a string is a palindrome.
// A palindrome reads the same forward and backward, ignoring case,
// whitespace, and punctuation.
// For example, "A man, a plan, a canal: Panama" is a palindrome.

fn is_palindrome(text: &str) -> bool {
    // TODO: Implement this function.
    // Check if the string is a palindrome, ignoring case, whitespace, and punctuation.
    // Return true if it is a palindrome, false otherwise.
    
    false // Replace this placeholder
}

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_join_strings() {
        let input = vec![
            String::from("Hello"),
            String::from("Rust"),
            String::from("World"),
        ];
        assert_eq!(join_strings(input), "Hello Rust World");
        
        let empty: Vec<String> = Vec::new();
        assert_eq!(join_strings(empty), "");
        
        let single = vec![String::from("Rust")];
        assert_eq!(join_strings(single), "Rust");
    }
    
    #[test]
    fn test_transform_sentence() {
        assert_eq!(
            transform_sentence("hello   rust   world"),
            "Hello Rust World"
        );
        
        assert_eq!(
            transform_sentence("rust is RUST and Rust and RusT"),
            "Rust Is Rust And Rust And Rust"
        );
        
        assert_eq!(
            transform_sentence("   extra    spaces   "),
            "Extra Spaces"
        );
    }
    
    #[test]
    fn test_count_characters() {
        let counts = count_characters("Hello, World!");
        assert_eq!(counts.get(&'h'), Some(&1));
        assert_eq!(counts.get(&'l'), Some(&3));
        assert_eq!(counts.get(&'o'), Some(&2));
        assert_eq!(counts.get(&','), Some(&1));
        
        let counts = count_characters("aAaA bBbB");
        assert_eq!(counts.get(&'a'), Some(&4));
        assert_eq!(counts.get(&'b'), Some(&4));
        assert_eq!(counts.get(&' '), Some(&1));
    }
    
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("Was it a car or a cat I saw?"));
        assert!(is_palindrome("No 'x' in Nixon"));
        
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("Rust"));
        assert!(!is_palindrome("OpenAI"));
    }
} 