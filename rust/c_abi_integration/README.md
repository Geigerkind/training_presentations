# Integrating C/C++ functions into Rust using the Foreign Function Interface (FFI)
> There is no point in reinventing the wheel and be all [RIIR](https://github.com/ansuz/RIIR) about it.

TODO: We hook into C ABI (Also used in python, ruby, etc.)

## Write your important library functions in C/C++
### Provide header files for the library / modify them, to not mangle functions
```c++
extern "C" int get_truth();
```
### The library itself
```c++
int get_truth() {
    return 42;
}
```
## Expose functions to Rust
```rust
extern {
    fn get_truth() -> i32;
}

fn main() {
    unsafe {
        println!("The truth is {}!", get_truth());
    }
}
```

## Use rusts toolchain to link Libraries with rust
```rust
extern crate cc;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=external/important_library.cpp");
    println!("cargo:rerun-if-changed=external/important_library.hpp");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("external/important_library.cpp")
        .include("external/")
        .cpp(true) // CPP compilation
        .compile("important_library");
}
```