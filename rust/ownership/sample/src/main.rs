fn set_truth(mut value: u32) -> u32 {
    value *= 2*3;
    value
}

fn main() {
    let mut answer = 7;
    println!("Current answer is {}", answer);
    answer = set_truth(answer);
    println!("Answer after changing {}", answer);
}
