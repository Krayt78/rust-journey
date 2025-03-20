# Data Types in Rust

Rust is a statically typed language, which means it must know the types of all variables at compile time. The compiler can usually infer the type based on the value and how it's used, but sometimes we need to add type annotations.

## Type Categories

Rust data types are divided into two main categories:

1. **Scalar Types**: Represent a single value
   - Integers
   - Floating-point numbers
   - Booleans
   - Characters

2. **Compound Types**: Group multiple values into one type
   - Tuples
   - Arrays

## Directory Structure

- [`01_scalar_types/`](./01_scalar_types/): Examples of integers, floats, booleans, and characters
- [`02_compound_types/`](./02_compound_types/): Examples of tuples and arrays

## Why Types Matter

Rust's type system helps prevent many common bugs found in other languages. By checking types at compile time, Rust ensures that your program will not try to perform invalid operations like treating a string as a number.

Choosing the right type for your data helps make your code:
- More efficient (using the appropriate amount of memory)
- More secure (preventing overflow and other issues)
- More maintainable (making your intentions clearer)

## Working Through This Section

1. Start with the scalar types in the [`01_scalar_types/`](./01_scalar_types/) directory
2. Move on to compound types in the [`02_compound_types/`](./02_compound_types/) directory
3. Complete the challenges in each file

Each file contains examples and explanations of the type along with challenges to test your understanding.