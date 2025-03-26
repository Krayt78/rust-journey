# Scalar Types in Rust

Scalar types in Rust represent single values. There are four primary scalar types:

## Integers

Integers are whole numbers without a fractional component. Rust has a variety of integer types with different sizes and signedness.

Key features of integers:
- Signed integers (i8, i16, i32, i64, i128, isize) can represent both positive and negative values
- Unsigned integers (u8, u16, u32, u64, u128, usize) can only represent non-negative values
- The default integer type is i32
- The number after 'i' or 'u' indicates how many bits are used to store the number

See the [0_integers.rs](./0_integers.rs) file for examples and exercises.

## Floating-Point Numbers

Floating-point numbers are numbers with decimal points. Rust has two floating-point types: f32 and f64.

Key features of floating-point numbers:
- f32: 32-bit floating point (single precision)
- f64: 64-bit floating point (double precision, default type)
- Can represent a wide range of decimal values
- Subject to precision limitations due to their binary representation

See the [1_floating_point.rs](./1_floating_point.rs) file for examples and exercises.

## Booleans

The boolean type in Rust has two possible values: true and false.

Key features of booleans:
- Size of one byte
- Used for conditional expressions and control flow
- Support logical operations (AND, OR, NOT)

See the [2_boolean.rs](./2_boolean.rs) file for examples and exercises.

## Characters

The char type in Rust represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

Key features of characters:
- Represented using single quotes (e.g., 'a', 'Z', '😀')
- Four bytes in size (supports Unicode)
- Can represent alphabetic characters, numeric digits, symbols, emojis, and more

See the [3_character.rs](./3_character.rs) file for examples and exercises.

## Working Through This Section

1. Start with [0_integers.rs](./0_integers.rs) to learn about whole numbers
2. Move on to [1_floating_point.rs](./1_floating_point.rs) to understand decimal values
3. Continue with [2_boolean.rs](./2_boolean.rs) to learn about logical values
4. Finally, explore [3_character.rs](./3_character.rs) to understand Unicode character representation
5. Complete the challenges in each file to test your understanding 