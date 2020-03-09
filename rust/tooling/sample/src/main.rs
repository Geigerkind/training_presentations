#[cfg(test)]
mod tests;
mod adder;

fn main() {
    let a: u32 = 41;
    let b: u32 = 1;
    println!("a + b = {}", adder::add(a, b));
}
