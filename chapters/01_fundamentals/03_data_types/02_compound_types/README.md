# Compound Types in Rust

Compound types in Rust can group multiple values into one type. The primary compound types in Rust are:

## Tuples

Tuples are a way of grouping together values of different types into one compound type. They have a fixed length and each position in the tuple has a specific type.

Key features of tuples:
- Fixed length - cannot grow or shrink once declared
- Can contain elements of different types
- Elements are accessed using dot notation with a zero-based index (e.g., `tuple.0`, `tuple.1`)
- Can be destructured with pattern matching
- Useful for returning multiple values from functions

See the [0_tuples.rs](./0_tuples.rs) file for examples and exercises.

## Arrays

Arrays are collections of multiple values of the same type. Unlike some other languages, arrays in Rust have a fixed length.

Key features of arrays:
- Fixed length - must be known at compile time
- All elements must be of the same type
- Elements are accessed using square bracket notation with a zero-based index (e.g., `array[0]`, `array[1]`)
- Stored on the stack (not the heap)
- The type signature is `[T; N]` where `T` is the type of each element and `N` is the fixed length

See the [1_arrays.rs](./1_arrays.rs) file for examples and exercises.

## Slices

Slices are references to a contiguous sequence of elements in a collection. They don't own data—they borrow it.

Key features of slices:
- Reference a portion of another data structure (like arrays or strings)
- Don't have ownership of the data they reference
- Have a length that can be determined at runtime
- Type signature is `&[T]` for immutable slices and `&mut [T]` for mutable slices
- String slices are a special type of slice that reference parts of a string (`&str`)
- Useful for functions that need to operate on portions of collections without taking ownership

See the [2_slices.rs](./2_slices.rs) file for examples and exercises.

## Difference from Scalar Types

Unlike scalar types (integers, floats, booleans, and characters) which hold only a single value, compound types can group multiple values together.

## When to Use Each Type

- **Tuples**: Use when you want to group together a small, fixed number of values that can be of different types. Common uses include returning multiple values from a function or representing a simple record where each field has a specific meaning.

- **Arrays**: Use when you want a collection of elements of the same type with a fixed length that's known at compile time. Arrays are great for performance-critical code because they're stored on the stack.

- **Slices**: Use when you need to operate on a portion of a collection without taking ownership. They're particularly useful for functions that need to accept different sized collections, as they provide a uniform interface.

## Working Through This Section

1. Start with [0_tuples.rs](./0_tuples.rs) to learn about grouping values of different types
2. Move on to [1_arrays.rs](./1_arrays.rs) to learn about fixed-length collections of the same type
3. Continue with [2_slices.rs](./2_slices.rs) to understand how to work with portions of collections
4. Complete the challenges in each file to test your understanding 