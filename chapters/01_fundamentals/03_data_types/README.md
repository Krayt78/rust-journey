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

## Learning Through Challenges

Each file in this section has two parts:

1. **Teaching Section**: The top part of each file contains working examples that demonstrate the data type concepts with explanations in comments.

2. **Challenges Module**: A module at the bottom of each file with intentionally broken code that you need to fix. These challenges test your understanding of the concepts presented in the teaching section.

To complete the challenges:
1. Read through the teaching section to understand the concepts
2. Look at the broken code in the challenges module
3. Fix each issue to make the code compile and run correctly
4. The code contains validations that will tell you if your fixes are correct

## Working Through This Section

1. Start with the scalar types in the [`01_scalar_types/`](./01_scalar_types/) directory:
   - integers.rs: Learn about different integer types and overflow handling
   - floating_point.rs: Explore floating-point precision and special values
   - boolean.rs: Understand logical operations and control flow
   - character.rs: See how Rust handles Unicode characters

2. Move on to compound types in the [`02_compound_types/`](./02_compound_types/) directory:
   - tuples.rs: Group values of different types together
   - arrays.rs: Work with collections of the same type

Happy coding! Remember that making mistakes and fixing them is one of the best ways to learn programming concepts.