# Rust as an alternative to C and C++
![CPP Risiken](img/cpp_leg_blowup.jpg)

## Use-After-Free
```c++
#include <iostream>
#include <string>

class Dog {
    std::string mName;
public:
    Dog(std::string &&name) {
        mName = std::move(name);
    }

    void bark() {
        std::cout << "Wuff!" << std::endl;
    }
};

int main() {
    std::string name = "Snuffles";
    Dog *snuffles = new Dog(std::move(name));

    delete snuffles; // RIP
    
    snuffles->bark();
}
```

## Buffer overflow
```c++
#include <iostream>
#include <cstring>

int main() {
    int auth = 0;
    char password[10];
    std::cout << "Password: ";
    std::cin >> password;

    if (strcmp(password, "secret!") == 0) {
        auth = 1;
    } else {
        std::cout << "Wrong password!" << std::endl;
    }

    if (auth) {
        std::cout << "Logged in!" << std::endl;
    }
}
```

## Type Safe Languages
> Practitioners who invented type safety often meant just **“memory integrity”**, while theoreticians always meant **“execution integrity”**, and it is the latter that seems more relevant now
* Static and runtime checks
* Array bounds checking
* Garbage collection

# What is Rust?

## Goals
> Take, for example, **“systems-level” work** that deals with low-level details of **memory management, data representation, and concurrency**. Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to **avoid its infamous pitfalls**. And even those who practice it do so with caution, lest their code be open to **exploits, crashes, or corruption**.
* Eliminate pitfalls
* Friendly and polished set of tools
* "Dip down" into lower level, **without** taking unnecessary risks of crashes and **security holes**
* Reliable, efficient and memory efficient code
* Low risk parallelism

## Domain
* System programming (Drivers, Operating systems, etc.)
* Embedded systems
* Compiler
* Browser
* Multimedia (Codecs etc.)
* **Web Servers**
* **Business Applications**

## History
* 2006 Personal project of Mozilla employee Graydon Hoare
* 2009 Mozilla began sponsoring the project
* 2011 Rust compiler compiled itself
* 2012 Pre-Alpha release
* 2015 First stable release

## Hello World
```rust
fn main() {
    println!("Hellow World");
}
```

# Sources
* https://www.rust-lang.org/
* https://en.wikipedia.org/wiki/Rust_(programming_language)
* https://github.com/rust-lang/book