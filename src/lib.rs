/// Adds two integers together.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The sum of `a` and `b`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts one integer from another.
///
/// # Arguments
///
/// * `a` - The integer to subtract from.
/// * `b` - The integer to subtract.
///
/// # Returns
///
/// The result of subtracting `b` from `a`.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two integers together.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The product of `a` and `b`.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides one integer by another.
///
/// # Arguments
///
/// * `a` - The integer to divide.
/// * `b` - The integer to divide by.
///
/// # Panics
///
/// This function will panic if `b` is zero.
///
/// # Returns
///
/// The result of dividing `a` by `b`.
pub fn divide(a: i32, b: i32) -> i32 {
    if b != 0 {
        a / b
    } else {
        panic!("Cannot divide by zero")
    }
}
