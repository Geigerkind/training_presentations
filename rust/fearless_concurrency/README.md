# Fearless concurrency
> **Race condition:** Multiple computations access shared data in a way that their results depend on the sequence of accesses.

> **Deadlocks:** Two threads waiting for each other to finish using a resource the other thread has, preventing both threads from continuing.

## Multi-threading
> **1:1 Model** - *One* language thread on *one* OS thread.

> **M:N Model** - *M* language threads on *N* OS threads. 

### Using `spawn` and `join`
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
Will produce this:
```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

### Fearless resource usage in threads
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```
The program will fail to compile with this failure message:
```
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```
If we take the suggestion from the compiler, it will instead tell us this:
```
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

## Message passing
> **Slogen of the Go language:** "Do not communicate by sharing memory; instead, share memory by communicating."

> Imagine it like a river and communication via a message in a bottle. Once you send it down the river, you can't access the message anymore.
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
This will produce following output:
```
Got: hi
```
> **Note:** *mspc* as an acronym for *multiple producer, single consumer*.

## Shared-state communication
> **More complex:** You have to guarantee *Safety*, *Liveness* and *Fairness*.

> **Rust** knows which types are thread safe and which are not using the **marker traits `Send` and `Sync`**.

> **`Send`:** Allowing transference of ownership between threads.  
> **`Sync`:** Allowing access from multiple threads, e.g. `Mutex<T>`  

> **Be aware:** There are still no guarantees for not producing **deadlocks** using Rust.