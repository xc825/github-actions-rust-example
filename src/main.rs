use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

fn main() {
    let a = 10;
    let b = 2;

    println!("Add: {}", add(a, b));
    println!("Subtract: {}", subtract(a, b));
    println!("Multiply: {}", multiply(a, b));
    println!("Divide: {}", divide(a, b));
}
