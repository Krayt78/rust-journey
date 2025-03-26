// Return Values in Rust Functions
//
// This program demonstrates how functions can return values.


// TODO: Fix this function to return the square of the input number
fn square(num: i32) {
    num * num;
}

// TODO: Fix this function to return a boolean indicating if a number is even
fn is_even(num: i32) {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

// TODO: Fix this function to return both the sum and product of two numbers
fn calculate(a: i32, b: i32) -> (i32, i32) {
    let sum = a + b;
    let product = a * b;
    // Something is wrong with this return statement
    sum, product
}

// TODO: Fix this function to find the maximum of three numbers
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    // This implementation is incorrect
    if a > b {
        a
    } else {
        b
    }
    // What about c?
}

fn main() {
}