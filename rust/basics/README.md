# Basics
## Variables
```rust
let truth: u8 = 42;
let mut mutable_truth: u8 = 21;
mutable_truth *= 2;
```

## Control flow
```rust
let answer = 42;
if answer == 42 || answer == 21 {
    println!("The answer is the truth: {}!", answer);
} else if answer == 2 || answer == 3 || answer == 7 {
    println!("The answer is part of the truth: {}", answer);
} else {
    println!("Wrong, just wrong");
}
```

## Functions
```rust
fn compute_length(input: &str) -> usize {
    input.len()
}
```