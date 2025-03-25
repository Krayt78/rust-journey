// Lifetimes in Rust
//
// This file demonstrates how lifetimes work in Rust and how
// they're used to prevent dangling references.

fn main() {
    println!("Understanding Lifetimes in Rust!");
    
    //------------------------------------------------------
    // LIFETIME BASICS
    //------------------------------------------------------
    println!("\n=== Lifetime Basics ===");
    
    // Scopes and variable lifetimes
    {
        let x = 5;            // ----------+-- 'a
                              //           |
        println!("x: {}", x); //           |
    }                         // ----------+
    // println!("x: {}", x);  // Error: x no longer exists
    
    // References and their lifetimes
    let r;                    // ---------+-- 'a
                              //          |
    {                         //          |
        let x = 5;            // -+-- 'b  |
        r = &x;               //  |       |
    }                         // -+       |
    // println!("r: {}", r);  // Error: r refers to a value that is dropped
                              //          |
    println!("Comment out the error to compile."); // |
                              // ---------+
    
    //------------------------------------------------------
    // FUNCTION PARAMETER LIFETIMES
    //------------------------------------------------------
    println!("\n=== Function Parameter Lifetimes ===");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    // Passing references with compatible lifetimes
    let result = longest(string1.as_str(), string2);
    println!("Longest string: {}", result);
    
    // A more complex example with nested scopes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // string1 outlives string2, but the result's lifetime is tied to string2
        let result = longest(string1.as_str(), string2.as_str());
        println!("Longest string in nested scope: {}", result);
    }
    
    // This would cause an error because result's lifetime is tied to string2
    // which is dropped when its scope ends
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("Longest string: {}", result);  // Error!
    
    //------------------------------------------------------
    // LIFETIMES WITH STRUCTS
    //------------------------------------------------------
    println!("\n=== Lifetimes with Structs ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
    
    //------------------------------------------------------
    // LIFETIME ELISION RULES
    //------------------------------------------------------
    println!("\n=== Lifetime Elision Rules ===");
    
    // The following functions don't need explicit lifetimes due to elision rules
    let s = String::from("Hello, world!");
    
    // Rule 1: Each parameter that is a reference gets its own lifetime parameter
    // Rule 2: If there's exactly one input lifetime parameter, that lifetime is 
    //         assigned to all output lifetime parameters
    let word = first_word(&s);
    println!("First word: {}", word);
    
    // Rule 3: If there are multiple input lifetime parameters, but one of them is 
    //         &self or &mut self, the lifetime of self is assigned to all output 
    //         lifetime parameters
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("Announcement: {}", excerpt.announce_and_return_part());
    
    //------------------------------------------------------
    // 'STATIC LIFETIME
    //------------------------------------------------------
    println!("\n=== 'static Lifetime ===");
    
    // The 'static lifetime indicates a value that lives for the entire program duration
    let s: &'static str = "I have a 'static lifetime.";
    println!("{}", s);
    
    // String literals are stored in the binary and always have 'static lifetime
    let a_literal: &'static str = "Hello, world!";
    println!("{}", a_literal);
    
    //------------------------------------------------------
    // COMBINING FEATURES
    //------------------------------------------------------
    println!("\n=== Combining Generics, Trait Bounds, and Lifetimes ===");
    
    let s1 = String::from("abcd");
    let s2 = "xyz";
    
    // Using a function that combines generics, trait bounds, and lifetimes
    let result = longest_with_announcement(
        s1.as_str(),
        s2,
        "Today's announcement: The shortest of the two strings is chosen!"
    );
    println!("Result: {}", result);
}

// Function with an explicit lifetime parameter
// This function finds the longest of two string slices
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function that combines generics, trait bounds, and lifetimes
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function that benefits from lifetime elision rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Struct that holds a reference, requiring a lifetime annotation
struct Excerpt<'a> {
    part: &'a str,
}

// Implementation for a struct with a lifetime parameter
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    // This method benefits from the third elision rule
    fn announce_and_return_part(&self) -> &str {
        println!("Attention please: {}", self.part);
        self.part
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Implementing a Function with Lifetimes
//
// Complete the 'shortest' function that takes two string slices and returns
// the shorter one. The function should explicitly annotate lifetimes where needed.

// TODO: Implement the 'shortest' function
// fn shortest<...>(...) -> ... {
//     ...
// }

// Exercise 2: Creating a Struct with Lifetimes
//
// Define a struct called 'TextAnalyzer' that holds references to:
// - original_text: a string slice containing the full text
// - current_word: a string slice that is a reference to a part of the original_text
// 
// Implement a method 'next_word' that updates current_word to the next word in original_text.
// If there are no more words, it should set current_word to an empty string.

// TODO: Define the TextAnalyzer struct
// struct TextAnalyzer<...> {
//     ...
// }

// TODO: Implement methods for TextAnalyzer
// impl<...> TextAnalyzer<...> {
//     fn new(...) -> Self {
//         ...
//     }
//     
//     fn next_word(&mut self) {
//         ...
//     }
//     
//     // Add a method to get the current word
//     fn current_word(&self) -> &str {
//         ...
//     }
// }

// Exercise 3: Generic Struct with Lifetime Bounds
//
// Create a generic struct called 'Cache' that holds:
// - A value of generic type T
// - A reference to a configuration of type &'a Config
// 
// Also define a struct 'Config' with some fields.
// Implement a method for Cache that returns a formatted string
// containing both the value and the configuration.

// TODO: Define the Config struct
// struct Config {
//     ...
// }

// TODO: Define the Cache struct with the appropriate lifetime parameter
// struct Cache<...> {
//     ...
// }

// TODO: Implement methods for Cache
// impl<...> Cache<...> {
//     fn new(...) -> Self {
//         ...
//     }
//     
//     fn format_with_config(&self) -> String {
//         ...
//     }
// }

// Exercise 4: Lifetime in Trait Implementation
//
// Define a trait called 'Summarizable' with a method 'summary' that returns a string.
// Implement this trait for a struct called 'Article' that contains references with different lifetimes.
// The Article should contain:
// - title: a string slice with lifetime 'a
// - author: a string slice with lifetime 'b
// - content: a string slice with lifetime 'a
//
// The summary method should return a string that combines these fields.

// TODO: Define the Summarizable trait
// trait Summarizable {
//     ...
// }

// TODO: Define the Article struct with appropriate lifetime parameters
// struct Article<...> {
//     ...
// }

// TODO: Implement Summarizable for Article
// impl<...> Summarizable for Article<...> {
//     ...
// }

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Exercise 1
    #[test]
    fn test_shortest() {
        let string1 = String::from("long");
        let string2 = String::from("longer");
        
        assert_eq!(shortest(string1.as_str(), string2.as_str()), "long");
        
        let string3 = String::from("a");
        let string4 = String::from("ab");
        
        assert_eq!(shortest(string3.as_str(), string4.as_str()), "a");
    }

    // Tests for Exercise 2
    #[test]
    fn test_text_analyzer() {
        let text = String::from("hello world rust programming");
        let mut analyzer = TextAnalyzer::new(text.as_str());
        
        assert_eq!(analyzer.current_word(), "hello");
        
        analyzer.next_word();
        assert_eq!(analyzer.current_word(), "world");
        
        analyzer.next_word();
        assert_eq!(analyzer.current_word(), "rust");
        
        analyzer.next_word();
        assert_eq!(analyzer.current_word(), "programming");
        
        analyzer.next_word();
        assert_eq!(analyzer.current_word(), "");
    }

    // Tests for Exercise 3
    #[test]
    fn test_cache() {
        let config = Config {
            app_name: String::from("MyApp"),
            version: String::from("1.0"),
            debug_mode: true,
        };
        
        let cache = Cache::new(42, &config);
        
        let result = cache.format_with_config();
        assert!(result.contains("42"));
        assert!(result.contains("MyApp"));
        assert!(result.contains("1.0"));
    }

    // Tests for Exercise 4
    #[test]
    fn test_article_summary() {
        let title = String::from("Understanding Rust");
        let author = String::from("Rustacean");
        let content = String::from("Rust is a systems programming language...");
        
        let article = Article {
            title: &title,
            author: &author,
            content: &content,
        };
        
        let summary = article.summary();
        assert!(summary.contains(&title));
        assert!(summary.contains(&author));
        assert!(summary.contains("Rust is a"));  // Just testing part of content
    }
} 