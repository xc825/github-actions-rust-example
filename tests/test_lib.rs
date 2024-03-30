use trust::{add, divide, multiply, subtract}; // Import the necessary functions
#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        assert_eq!(trust::add(2, 2), 4);
        assert_eq!(trust::add(0, 0), 0);
        assert_eq!(trust::add(-2, 2), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(trust::subtract(2, 2), 0);
        assert_eq!(trust::subtract(0, 0), 0);
        assert_eq!(trust::subtract(-2, 2), -4);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(trust::multiply(2, 2), 4);
        assert_eq!(trust::multiply(0, 0), 0);
        assert_eq!(trust::multiply(-2, 2), -4);
    }

    #[test]
    fn test_divide() {
        assert_eq!(trust::divide(2, 2), 1);
        assert_eq!(trust::divide(-2, 2), -1);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        trust::divide(2, 0);
    }
}
