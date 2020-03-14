#[test]
fn division_by_zero() {
    let dividend = 42.0;
    let divisor = 0.0;

    let result = divide(dividend, divisor);
    assert!(result.is_err());
}

#[test]
fn division_valid() {
    let dividend = 42.0;
    let divisor = 2.0;

    let result = divide(dividend, divisor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 21.0);
}


fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        return Err("Divisor is 0".to_string());
    }
    Ok(dividend / divisor)
}

fn main() {
    let dividend = 84.0;
    let divisor = 2.0;
    println!("Result of {} and {} is {}", dividend, divisor, divide(dividend, divisor).unwrap());
}
