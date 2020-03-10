# Tooling
## Cargo
> Cargo is Rust’s **build system** and **package manager**.

> **Unified** tooling.

### Dependency management
```toml
[package]
name = "rust_testing_example"
version = "0.1.0"
authors = ["Tom Dymel <td@wps.de>"]
edition = "2018"

[dependencies]

[dev-dependencies]
proptest = "0.9.5"
csv = "1.1"
mutagen = { git="https://github.com/llogiq/mutagen" }
```

### Command line interface

```shell-script
cargo update
cargo clean
cargo build --release
cargo run
# etc.
```

## Tests
```shell-script
cargo test
```

### Mark modules to be test only
```rust
#[cfg(test)]
mod tests;
```

### Specify test functions
```rust
#[test]
fn division_by_zero() {
    let dividend = 42.0;
    let divisor = 0.0;

    let result = divide(dividend, divisor);
    assert!(result.is_err());
}
```

### Third party testing frameworks and testing tools
> See [this repository](https://github.com/Geigerkind/Rust-testing-example) for more information.

## Documentation
> cargo-doc - Build a package's documentation
```shell-script
cargo doc --open
```
### Inline documentation
```rust
/// # Fancy Header
/// > Prev(Succ(None)) = None  
/// Adds two unsigned integers a and b  
/// * You can use   
/// * Any kind  
/// * of mark down here  
pub fn add(a:u32, b:u32) -> u32 {
    a + b
}
```

## Formatting
> Formats the code according to the **official** Rust style guidelines.
```shell-script
rustup component add rustfmt
cargo fmt
```

## Linting with Clippy
> A collection of lints to catch common mistakes and improve your Rust code.  
> [There are 360 lints included in this crate!](https://rust-lang.github.io/rust-clippy/master/index.html)
```shell-script
rustup component add clippy
cargo clippy
cargo fix -Z unstable-options --clippy
```

## Other Tools
* Rust provides a **language server** => Language support, e.g. Code and IntelliJ
* Easy continues integration
* Build scripts ([See integration of C/C++ libraries](https://github.com/Geigerkind/training_presentations/tree/master/rust/c_abi_integration))

## Macros
> Write Rust programs that manipulate Rust programs.