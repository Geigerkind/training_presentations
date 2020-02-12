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