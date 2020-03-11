# Code structuring
* **Packages:** Build, test, share *crates* with Cargo
* **Crates:** A tree of *modules*
* **Modules:** Organization, scope, and privacy of *paths*
* **Paths:** Naming an struct, function, or module

## Modules
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## Paths
```rust
fn main() {
    // Absolute path
    crate::front_of_house::hosting::seat_at_table();

    // Relative path
    // see also, self:: and super::
    front_of_house::serving::take_order();
}

```

## Scopes
> **Private** by default, expose using `pub` keyword. 

## Separating modules into different folders and files
> **File names** and **directory names** specify **modules**

> Folders require special **mod** file

```rust
use front_of_house::serving;

mod front_of_house;

fn main() {
    serving::take_order();
}

```
> front_of_house is a **folder** with two files **serving** and **hosting**

# Exercise
1. Create a `tools` and a `tests` folder.
2. Extract a file for the fibonacci method.
3. Extract a file for the tests of the fibonacci method.
4. Print some value of the fibonacci sequence in the `main` function (using the predefined function).
5. See if the tests are still running.