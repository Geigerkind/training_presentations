use crate::adder;

#[test]
fn test_add_with_zero() {
    let a = 2;
    let b = 0;

    let result = adder::add(a, b);
    assert_eq!(result, a);
}