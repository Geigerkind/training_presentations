# Basics
## Types
```rust
// Integer
let signed_char: i8;
let unsigned_char: u8;
let signed_smallint: i16;
let unsigned_smallint: u16;
let signed_integer: i32;
let unsigned_integer: u32;
let signed_bigint: i64;
let unsigned_bigint: u64;
let signed_long_bigint: i128;
let unsigned_long_bigint: u128;

// Boolean
let bool_var: bool;

// Floats
let float_32: f32;
let float_64: f64;

// Arrays
// Fixed size
let bytes: [u8; 42];
// Variable size
let byte_vec: Vec<u8>;

// Map
let hashmap: HashMap<u32, String>;

// String
// static
let static_string: &str = "I am a static string";
let normal_string: String = String::from(static_string);
```

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

## Loops
```rust
for i in 0.100 {
    // Do something
}

while true {
    // Do sth
}
```

## Useful macros
```rust
fn main() {
    let name = format!("{1}{0}", "rge", "Geo");
    println!("Hello, {}! The magic number is {}!", name, 42);
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
