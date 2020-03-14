# Rust - Tour of Heroes
<div style="text-align: center">
    <img src="./img/logo.png" alt="drawing" width="500"/>
</div>

# Installation
You will need this software!

## Software and compiler
### Windows
* [RustUp](https://rustup.rs/#)
* [Docker](https://docs.docker.com/docker-for-windows/)
* [NPM](https://nodejs.org/en/download/)
* [VS Build Tools](https://visualstudio.microsoft.com/vs/)
    * C++ Build tools for desktop
    * Select: MSVC and Windows 10 SDK

### MAC
* [RustUp](https://rustup.rs/#)
* [Docker](https://docs.docker.com/docker-for-mac/install/)
* [NPM](https://nodejs.org/en/download/)

### Linux
```shell-script
yay -S rustup docker docker-compose npm
systemctl start docker
```

## Further configuration
```shell-script
# The windows installer will ask you, but you can also use the CLI afterwards
rustup install nightly
rustup default nightly
```

# Agenda
* [Introduction](https://github.com/Geigerkind/rust-schulung/tree/master/rust/introduction)
* [Basics](https://github.com/Geigerkind/rust-schulung/tree/master/rust/basics)
* [Tooling](https://github.com/Geigerkind/rust-schulung/tree/master/rust/tooling)
* [Code structuring](https://github.com/Geigerkind/rust-schulung/tree/master/rust/code_structuring)
* [Ownership](https://github.com/Geigerkind/rust-schulung/tree/master/rust/ownership) and [borrowing](https://github.com/Geigerkind/rust-schulung/tree/master/rust/borrowing)
* [Structs and enums](https://github.com/Geigerkind/rust-schulung/tree/master/rust/structs_and_enums)
* [Traits](https://github.com/Geigerkind/rust-schulung/tree/master/rust/traits)
* [OOP in Rust](https://github.com/Geigerkind/rust-schulung/tree/master/rust/oop_in_rust)
* [Error handling](https://github.com/Geigerkind/rust-schulung/tree/master/rust/error_handling)
* [A Tour Of Heroes](https://github.com/Geigerkind/training_presentations/tree/master/rust/a_hero_starter_pack)
* [Fearless concurrency](https://github.com/Geigerkind/rust-schulung/tree/master/rust/fearless_concurrency)
* [Asynchronous code](https://github.com/Geigerkind/rust-schulung/tree/master/rust/async_code)
* [Integrating C/C++ functions](https://github.com/Geigerkind/rust-schulung/tree/master/rust/c_abi_integration)
* [Long-term viability](https://github.com/Geigerkind/rust-schulung/tree/master/rust/long_term_viability)

# Sources
* https://www.rust-lang.org/
* https://github.com/rust-lang/book
* https://techdifferences.com/difference-between-stack-and-heap.html
* https://docs.rust-embedded.org/book/interoperability/c-with-rust.html
* https://github.com/alexcrichton/cc-rs
* https://youtu.be/DnT-LUQgc7s
* https://doc.rust-lang.org/book/ch16-00-concurrency.html
* https://blog.rust-lang.org/inside-rust/2020/03/04/recent-future-pattern-matching-improvements.html
* https://doc.rust-lang.org/cargo/index.html
* https://github.com/rust-lang/rust-clippy
* https://github.com/rust-lang/rustfmt
* https://rust-lang.github.io/async-book/