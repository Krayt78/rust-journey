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
    
    // Challenge: Create a function that determines if a character is a vowel
    // Your code here:
    
    // Challenge: Write a program that takes a character and prints its ASCII value
    // and whether it is uppercase, lowercase, numeric, or a symbol
    // Your code here:
} 