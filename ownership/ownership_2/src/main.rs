fn say_name(name: String) {
    println!("{}", name);
}

fn main() {
    let name = String::from("Snuffles");
    say_name(name);
    println!("say_name was called with: {}", name);
}