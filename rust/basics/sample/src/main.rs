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
    (0..100).for_each(|n| println!("Fibonacci for n={0}: {1}", n, fibonacci(n)))
}
