use crate::tools::fibonacci;

#[test]
fn test_base_cases() {
    assert_eq!(fibonacci(-2), -1.0);
    assert_eq!(fibonacci(-1), 1.0);
    assert_eq!(fibonacci(0), 0.0);
    assert_eq!(fibonacci(1), 1.0);
    assert_eq!(fibonacci(2), 1.0);
}

#[test]
fn test_test_some_value() {
    assert_eq!(fibonacci(8), 21.0);
    assert_eq!(fibonacci(-8), -21.0);
}

#[test]
fn test_test_recursive_expansion() {
    assert_eq!(fibonacci(8), fibonacci(7) + fibonacci(6));
}
