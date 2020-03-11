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

## `println!` macro
```rust
fn main() {
    println!("Hello, {}! The magic number is {}!", "George", 42);
}
```

## Comments
```rust
// This is a single line comment
/*
* This is a block comment
*/
```

# Exercise
Create a new sample project:
```shell-script
cargo new fibonacci
cd fibonacci
code ./
```
Print the first 100 entries of the fibonacci sequence.
> `f_n = f_(n-1) + f_(n-2)`, where `f_1 = f_2 = 1` and `f_0 = 0`  
> `f_(-n) = (-1)^(n+1) * f_n`    
> **Example:** 0, 1, 1, 2, 3, 5, 8, 13, 21...