# Ownership
> Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.  
> 1. Each value in Rust has a variable that’s called its owner.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be dropped.

## Stack vs. Heap
![Stack vs. Heap](./img/stack_vs_heap.jpg)

## Example: C++ vs. Rust
### C++: dog2 is an alias for dog1
```c++
int main() {
    std::string *dog1 = new std::string("Snuffels");
    std::string *dog2 = dog1;

    std::cout << "Dog 1: " << *dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << *dog2 << " (" << &dog2 << ") " << std::endl;

    delete dog1;
}
```
<img src="./img/ownership_1_cpp.svg" alt="drawing" width="300"/>

### Rust: dog2 takes ownership of dog1
```rust
fn main() {
    let dog1 = String::from("Snuffles");
    let dog2 = dog1;

    println!("Dog 1: {} / Dog 2: {}", dog1, dog2);
}
```
<img src="./img/ownership_1_rust.svg" alt="drawing" width="300"/>

> Resource Acquisition Is Initialization (RAII)

> After leaving the scope where dog1/2 were initialized, **drop** will be called. This is similar to the destructor in C++.


### C++: dog2 takes ownership of dog1
```c++
int main() {
    std::string dog1 = "Snuffels";
    std::string dog2 = std::move(dog1);

    std::cout << "Dog 1: " << dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << dog2 << " (" << &dog2 << ") " << std::endl;
}
```

## Functions
```rust
fn say_name(name: String) {
    prinln!("{}", name);
}

fn main() {
    let name = String::from("Snuffles");
    say_name(name);
    println!("say_name was called with: {}", name);
}
```

```c++
void say_name(std::string &&name) {
    std::cout << name << std::endl;
}

int main() {
    std::string *dog1 = new std::string("Snuffels");
    std::string *dog2 = dog1;

    say_name(std::move(*dog1));
    say_name(std::move(*dog2));

    std::cout << "Dog 1: " << *dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << *dog2 << " (" << &dog2 << ") " << std::endl;

    delete dog1;
}
```

# Exercise
1. Create a function that takes ownership
2. Does something with it
3. And then returns it back