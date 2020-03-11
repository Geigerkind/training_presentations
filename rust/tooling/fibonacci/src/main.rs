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

/// # Fibonacci sequence
/// f_n = f_(n-1) + f_(n-2)  
/// f_0 = 0  
/// f_1 = f_2 = 1  
/// f_(-n) = (-1)^(n+1)*f_n
fn fibonacci(until: i32) -> f64 {
    if until == 0 {
        return 0.0;
    }

    let until_abs = until.abs();
    let mut res_n1: f64 = 1.0;
    let mut res_n2: f64 = 0.0;
    for _i in 1..until_abs {
        let new_res = res_n1 + res_n2;
        res_n1 = res_n2;
        res_n2 = new_res;
    }

    let res = res_n1 + res_n2;
    if until < 0 && until_abs % 2 == 0 {
        return -1.0 * res;
    }
    res
}

fn main() {
    let n=8;
    println!("Fibonacci of n={0} is {1}", n, fibonacci(n));
}
