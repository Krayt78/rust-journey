// Character Type in Rust
//
// The char type in Rust represents a Unicode Scalar Value,
// which means it can represent a lot more than just ASCII.
// A char is four bytes in size and represents a single Unicode character.
//
// For more information, see The Rust Book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type

fn main() {
    println!("Exploring Character Types in Rust!");
    
    //------------------------------------------------------
    // BASIC CHARACTER DECLARATIONS
    //------------------------------------------------------
    // Character declaration with single quotes
    // Note: strings use double quotes, chars use single quotes
    let c = 'z';
    let z: char = 'ℤ'; // explicit type annotation
    let heart_eyed_cat = '😻';
    
    println!("Basic characters:");
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    
    // Chars take up 4 bytes (32 bits) in Rust because they represent
    // Unicode Scalar Values (U+0000 to U+D7FF and U+E000 to U+10FFFF)
    println!("\nSize of char: {} bytes", std::mem::size_of::<char>());
    
    //------------------------------------------------------
    // ASCII CHARACTERS
    //------------------------------------------------------
    // ASCII characters
    let ascii_a = 'A';
    println!("\nASCII character: {}", ascii_a);
    println!("ASCII value of '{}': {}", ascii_a, ascii_a as u32);
    
    //------------------------------------------------------
    // SPECIAL CHARACTERS AND ESCAPES
    //------------------------------------------------------
    // Special characters and escapes
    let newline = '\n';
    let tab = '\t';
    let backslash = '\\';
    let single_quote = '\'';
    
    println!("\nSpecial characters:");
    println!("Newline as escaped string: '{}'", newline);
    println!("Tab displayed: '{}tabs here'", tab);
    println!("Backslash character: '{}'", backslash);
    println!("Single quote character: '{}'", single_quote);
    
    //------------------------------------------------------
    // UNICODE CHARACTERS
    //------------------------------------------------------
    // Unicode characters
    let pi = 'π';
    let emoji = '🦀'; // Rust's mascot!
    
    println!("\nUnicode characters:");
    println!("Pi symbol: {}", pi);
    println!("Rust emoji: {}", emoji);
    
    //------------------------------------------------------
    // UNICODE ESCAPES
    //------------------------------------------------------
    // Unicode escape sequences
    let unicode_escape = '\u{1F980}'; // Unicode for 🦀
    println!("\nUnicode escape sequence '\\u{{1F980}}': {}", unicode_escape);
    
    //------------------------------------------------------
    // CHARACTER METHODS
    //------------------------------------------------------
    // Character methods and properties
    let a = 'A';
    let b = '9';
    let c = ' ';
    let d = '😀';
    
    println!("\nCharacter methods:");
    println!("'{}' is alphabetic: {}", a, a.is_alphabetic());
    println!("'{}' is numeric: {}", b, b.is_numeric());
    println!("'{}' is whitespace: {}", c, c.is_whitespace());
    println!("'{}' is alphanumeric: {}", d, d.is_alphanumeric());
    
    //------------------------------------------------------
    // CHARACTER CONVERSION
    //------------------------------------------------------
    // Converting between char and integer
    let char_code = 65; // ASCII code for 'A'
    let char_from_code = char::from_u32(char_code).unwrap();
    
    println!("\nCharacter conversion:");
    println!("Character from code {}: '{}'", char_code, char_from_code);
    println!("Code from character '{}': {}", 'Z', 'Z' as u32);
}

//------------------------------------------------------
// CHALLENGES: TODO: Fix the broken code in this module
//------------------------------------------------------
mod challenges {
    // TODO: These character declarations have errors
    pub fn challenge_character_declarations() -> Result<(), String> {
        // Error 1: Using double quotes instead of single quotes
        let a = "A";
        
        // Error 2: Trying to assign multiple characters to a char
        let greeting = 'hi';
        
        // Error 3: Missing escape for special character
        let quote = ''';
        
        // Uncomment and fix these declarations
        // let fixed_a = ...
        // let fixed_greeting = ...
        // let fixed_quote = ...
        
        // Don't modify below this line
        Ok(())
    }
    
    // TODO: This function should determine if a character is a vowel
    pub fn is_vowel(c: char) -> bool {
        // This check is incorrect - fix it to properly identify vowels
        c == 'a' && c == 'e'
    }
    
    // TODO: This function should return information about a character
    pub fn character_info(c: char) -> String {
        // This implementation has multiple errors - fix them
        let category = if c.is_alphabetic {
            "an alphabetic character"
        } else if c.is_numeric {
            "a numeric character"
        } else if c.is_whitespace {
            "a whitespace character"
        } else {
            "a special character"
        };
        
        let case = if c.is_uppercase {
            "uppercase"
        } else if c.is_lowercase {
            "lowercase"
        } else {
            "neither uppercase nor lowercase"
        };
        
        // Return the character information
        "Character '" + c + "' is " + category + " and is " + case
    }
}

//------------------------------------------------------
// Tests for the challenges
//------------------------------------------------------
mod tests {
    use super::challenges;
    
    #[test]
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning character challenges...");
        
        // Challenge 1: Fix character syntax
        challenge_character_syntax()?;
        println!("Successfully fixed character declarations!");
        
        // Challenge 2: Fix the is_vowel function
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let non_vowels = ['b', 'c', 'd', 'f', 'g', 'x', 'y', 'z', '1', ' '];
        
        for &vowel in &vowels {
            if !challenges::is_vowel(vowel) {
                return Err(format!("is_vowel('{}') should return true", vowel));
            }
        }
        
        for &non_vowel in &non_vowels {
            if challenges::is_vowel(non_vowel) {
                return Err(format!("is_vowel('{}') should return false", non_vowel));
            }
        }
        println!("Successfully fixed the vowel check function!");
        
        // Challenge 3: Fix the character_info function
        let info_a = challenges::character_info('A');
        if !info_a.contains("alphabetic") || !info_a.contains("uppercase") {
            return Err(format!("character_info('A') returned incorrect info: {}", info_a));
        }
        
        let info_9 = challenges::character_info('9');
        if !info_9.contains("numeric") {
            return Err(format!("character_info('9') returned incorrect info: {}", info_9));
        }
        
        println!("Successfully fixed the character_info function!");
        println!("All character challenges completed successfully!");
        Ok(())
    }
    
    #[test]
    fn challenge_character_syntax() -> Result<(), String> {
        // Since we can't directly access the variables in the challenge functions,
        // we just call the function and assume it's fixed if it doesn't panic
        challenges::challenge_character_declarations()
    }
} 