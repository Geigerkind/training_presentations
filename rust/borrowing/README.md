# References & Borrowing
> * At any given time, you can have either one mutable reference or any number of immutable references.
> * References must always be valid.

## Passing a variable to a function without loosing ownership
```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
```
<img src="./img/borrowing.svg" alt="drawing" width="500"/>

## Explicit mutability instead of implicit mutability
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

```

## Either one mutable refrence or many immutable references
### Prevent data races at compile time!
```rust 
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

### Anyone who is reading a value would not expect the value to suddenly change
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

## Dangling References
### C++: Will compile just fine
```c++
int* dangling() {
    auto a = std::make_unique<int>(2*3*7);
    return a.get();
}

int main() {
    int *answer = dangling();
    std::cout << "Address of answer: " << answer << std::endl;
    std::cout << "Answer is: " << *answer << std::endl;
}
```

### Rust: Will complain during compilation
```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let reference_to_nothing = dangle();
}
```

# Exercise
1. Create a function that borrows a variable and does something with it
2. Attempt to borrow it to multiple variables
3. Play around with mutability