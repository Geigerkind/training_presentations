# Error handling
> **Explicit** error handling.

## Unrecoverable errors
> Rust does **not** have exceptions.
```rust
fn main() {
    panic!("crash and burn");
}
```
When you run the program, you’ll see something like this:
```shell-script
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

## Recoverable errors
> Errors not serious enough to require the program to stop entirely.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
> Developer has to explictly *not care* about a possible failure by using `unwrap`.

### Propagating errors
#### Verbose version
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
#### Syntactic sugar
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

# Exercise
1. Write a `divide` function that throws an `Err` if we divide by 0, or otherwise returns a result with the computation
2. Write tests with this behavior