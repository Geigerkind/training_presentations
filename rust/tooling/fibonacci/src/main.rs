use tools::fibonacci;

#[cfg(test)]
mod tests;
mod tools;

fn main() {
    let n = 8;
    println!("Fibonacci of n={0} is {1}", n, fibonacci(n));
}
