fn main() {
    let mut res_n1: f64 = 1.0;
    let mut res_n2: f64 = 0.0;
    for i in 0..100 {
        let new_res = res_n1 + res_n2;
        res_n1 = res_n2.clone();
        res_n2 = new_res.clone();
        println!("Fibonacci for n={1}: {0}", new_res, i+1);
    }
}
