extern {
    fn get_truth() -> i32;
}

fn main() {
    unsafe {
        println!("The truth is {}!", get_truth());
    }
}
