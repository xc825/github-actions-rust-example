use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

/// This is the main function.
///
/// It performs arithmetic operations using the `add`, `subtract`, `multiply`, and `divide` functions.
fn main() {
    let a = 10;
    let b = 2;

    println!("Add: {}", add(a, b));
    println!("Subtract: {}", subtract(a, b));
    println!("Multiply: {}", multiply(a, b));
    println!("Divide: {}", divide(a, b));
}
