// Character Type in Rust
//
// The char type in Rust represents a Unicode Scalar Value,
// which means it can represent a lot more than just ASCII.
// A char is four bytes in size and represents a single Unicode character.

fn main() {
    println!("Exploring Character Types in Rust!");
    
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
    
    // ASCII characters
    let ascii_a = 'A';
    println!("\nASCII character: {}", ascii_a);
    println!("ASCII value of '{}': {}", ascii_a, ascii_a as u32);
    
    // Special characters and escapes
    let tab = '\t';
    let newline = '\n';
    let single_quote = '\'';
    let backslash = '\\';
    
    println!("\nSpecial characters:");
    println!("Tab character: '{}' creates indentation", tab);
    println!("Newline character: '{}' creates a new line", newline);
    println!("Escaped single quote: {}", single_quote);
    println!("Escaped backslash: {}", backslash);
    
    // Unicode characters
    let unicode_char = '\u{1F602}'; // 😂 FACE WITH TEARS OF JOY
    println!("\nUnicode character: {}", unicode_char);
    
    // Converting between chars and integers
    let num_char = '9';
    let digit = num_char.to_digit(10).unwrap(); // Convert char to number in base 10
    println!("\nConverting character '{}' to number: {}", num_char, digit);
    
    // Converting integer to char
    let ascii_code = 65;
    let char_from_ascii = char::from_u32(ascii_code).unwrap();
    println!("Converting ASCII code {} to character: {}", ascii_code, char_from_ascii);
    
    // Character methods
    let letter = 'A';
    let number = '1';
    let symbol = '@';
    let space = ' ';
    
    println!("\nCharacter methods:");
    println!("'{}' is alphabetic: {}", letter, letter.is_alphabetic());
    println!("'{}' is numeric: {}", number, number.is_numeric());
    println!("'{}' is alphanumeric: {}", symbol, symbol.is_alphanumeric());
    println!("'{}' is whitespace: {}", space, space.is_whitespace());
    println!("'{}' is uppercase: {}", letter, letter.is_uppercase());
    println!("'{}' lowercase: {}", letter.to_lowercase());
    
    // Note: to_lowercase() returns an iterator, which is why the output might look odd
    // In real code you would collect it to a String
    println!("'{}' lowercase as string: {}", letter, letter.to_lowercase().to_string());
    
    // Character literals vs. string literals
    let char_a = 'a';    // Character literal (single quotes)
    let string_a = "a";  // String literal (double quotes)
    
    println!("\nCharacter vs String:");
    println!("char_a: {}, type: char, size: {} bytes", char_a, std::mem::size_of_val(&char_a));
    println!("string_a: {}, type: &str, size: {} bytes", string_a, std::mem::size_of_val(&string_a));
    
    println!("\n==== CHALLENGES ====");
    println!("Fix the code in the challenges module to make it compile and run correctly!");
    
    // Run the challenges if they compile
    if let Err(e) = tests::run_challenges() {
        println!("Challenge error: {}", e);
    }
}

// CHALLENGES: Fix the broken code in this module
mod challenges {
    // TODO: These character declarations have errors
    
    // Error 1: Using double quotes instead of single quotes for chars
    let letter = "B";
    
    // Error 2: Too many characters in a char literal
    let emoji = '😀😀';
    
    // Error 3: Missing escape for special character
    let star = \u{2605};
    
    // TODO: This function incorrectly checks for vowels
    pub fn is_vowel(c: char) -> bool {
        // This only checks for lowercase vowels and uses wrong syntax
        c == 'a' || c == e || c == 'i' || c == 'o' || c = 'u'
        // Should check for both uppercase and lowercase
    }
    
    // TODO: This function should return information about a character
    pub fn character_info(c: char) -> String {
        // This has multiple errors in the logic and syntax
        let info = String::new()
        
        // Add the ASCII value to the info string
        info.push_str("ASCII value: " + c as u32);
        
        // Check if uppercase
        if c.is_uppercase {
            info.push_str(", type: uppercase");
        }
        // Check if lowercase
        else if c.is_lowercase() {
            info.push_str(", type: lowercase");
        }
        // Check if numeric
        else if c.is_numeric {
            info.push_str(", type: numeric");
        }
        // Otherwise it's a symbol or other type
        else {
            info.push_str(", type: symbol or other");
        }
        
        info
    }
}

// Tests for the challenges
mod tests {
    use super::challenges;
    
    pub fn run_challenges() -> Result<(), String> {
        println!("\nRunning character challenges...");
        
        // Challenge 1: Fix the character declarations
        // The code below has several errors related to character syntax
        challenge_character_syntax()?;
        
        // Challenge 2: Fix the is_vowel function
        // The function should return true for vowels (a, e, i, o, u) in any case
        let test_char = 'a';
        if challenges::is_vowel(test_char) {
            println!("'{}' is a vowel!", test_char);
        } else {
            return Err(format!("is_vowel('{}') should return true", test_char));
        }
        
        // Challenge 3: Fix the character_info function
        // It should correctly identify the type of character
        let result = challenges::character_info('A');
        if !result.contains("uppercase") {
            return Err("character_info('A') should identify the character as uppercase".to_string());
        }
        
        println!("All character challenges completed successfully!");
        Ok(())
    }
    
    fn challenge_character_syntax() -> Result<(), String> {
        // This is just a placeholder as the real challenge is in the challenges module
        // The actual verification would need to be done differently since we can't access 
        // the local variables in the challenges module directly
        
        // For now, we'll just return Ok
        println!("Character syntax challenge completed!");
        Ok(())
    }
} 