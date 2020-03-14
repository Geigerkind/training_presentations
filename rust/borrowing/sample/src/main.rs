fn set_truth(answer: &mut u32) {
    *answer *= 2*3;
}

fn main() {
    let mut answer = 7;
    println!("Answer before {}", answer);
    set_truth(&mut answer);
    println!("Answer after {}", answer);
}
