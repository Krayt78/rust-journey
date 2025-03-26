println!("Let's explore integer types in Rust!");
    
    //------------------------------------------------------
    // SIGNED INTEGERS
    //------------------------------------------------------
    // Signed integers can store both positive and negative values
    let a: i8 = 127;    // Maximum value for i8
    let b: i8 = -128;   // Minimum value for i8
    println!("i8 range: {} to {}", b, a);
    
    //------------------------------------------------------
    // UNSIGNED INTEGERS
    //------------------------------------------------------
    // Unsigned integers can only store non-negative values
    let c: u8 = 255;    // Maximum value for u8
    let d: u8 = 0;      // Minimum value for u8
    println!("u8 range: {} to {}", d, c);
    
    //------------------------------------------------------
    // DEFAULT INTEGER TYPE
    //------------------------------------------------------
    // The default integer type in Rust is i32
    let e = 42;   // Type is inferred as i32
    println!("Default integer: {}", e);
    
    //------------------------------------------------------
    // ARCHITECTURE-DEPENDENT SIZES
    //------------------------------------------------------
    // isize and usize depend on the architecture of the computer
    // (64 bits on a 64-bit system, 32 bits on a 32-bit system)
    let f: isize = 9000;
    let g: usize = 9000;
    println!("isize: {}, usize: {}", f, g);
    
    //------------------------------------------------------
    // INTEGER LITERALS
    //------------------------------------------------------
    // Integer literals can have a suffix to specify their type
    let h = 42u8;    // u8
    let i = 1_000i64; // i64 - note the underscore for readability
    println!("Explicit types: {} (u8), {} (i64)", h, i);
    
    //------------------------------------------------------
    // NUMBER REPRESENTATIONS
    //------------------------------------------------------
    // Different number representations
    let decimal = 98_222;      // Decimal
    let hex = 0xff;           // Hex
    let octal = 0o77;         // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A';          // Byte (u8 only) - ASCII value of 'A'
    
    println!("Number representations:");
    println!("Decimal: {}", decimal);
    println!("Hex: {} (0xff)", hex);
    println!("Octal: {} (0o77)", octal);
    println!("Binary: {} (0b1111_0000)", binary);
    println!("Byte: {} (b'A' - ASCII of 'A')", byte);
    
    //------------------------------------------------------
    // INTEGER OVERFLOW
    //------------------------------------------------------
    // Integer overflow behavior
    // In debug builds, Rust includes checks that panic on overflow
    // In release builds, Rust performs "two's complement wrapping"
    
    // Uncomment to see overflow panic in debug mode:
    // let will_overflow: u8 = 255 + 1;
    
    // Ways to handle potential overflow:
    let x: u8 = 255;
    
    // Wrapping (two's complement)
    let wrapped = x.wrapping_add(1);
    
    // Return None if overflow occurs
    let checked = x.checked_add(1);
    
    // Return the value and a boolean indicating overflow
    let overflowed = x.overflowing_add(1);
    
    // Saturate at the min or max value
    let saturated = x.saturating_add(1);
    
    println!("\nHandling overflow for u8 (255):");
    println!("Wrapped: {}", wrapped);
    println!("Checked: {:?}", checked);
    println!("Overflowed: {:?}", overflowed);
    println!("Saturated: {}", saturated);