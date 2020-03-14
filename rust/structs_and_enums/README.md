# Structured Objects
```rust
pub struct Issue {
    pub id: u32,
    pub priority: IssuePriority,
    pub title: String,
    pub description: String,
    pub sub_tasks: Vec<String>
}

impl Issue {
    pub fn set_priority(&mut self, new_priority: IssuePriority) {
        self.priority = new_priority.to_owned();
    }
}

impl Default for Issue {
    fn default() -> Issue {
        Issue {
            id: 1,
            priority: IssuePriority::Bug,
            title: "".to_string(),
            description: "".to_string(),
            sub_tasks: vec![]
        }
    }
}
```

# Enumerations
> Rust’s enums are most similar to **algebraic data types** in functional languages, such as F#, OCaml, and Haskell.

> The most similar equivalent in C++ is the std::variant.

## Example: IssuePriority
```rust
enum IssuePriority {
    Enhancement,
    Bug,
    TODO,
    Milestone(u8)
    // Or named: Milestone { number: u8 }
}

fn main() {
    let mut milestone = Issue::default();
    milestone.set_priority(IssuePriority::Milestone(42));
}
```

## Option enum
### The Billion-dollar mistake
> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

### Rust does not have **null**
```rust
enum Option<T> {
    Some(T),
    None,
}
```
> Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

* Improves confidence in the code

## Pattern matching
### Match Control Flow Operator
```rust
fn print_issue_priority(priority: IssuePriority) {
    match priority {
        IssuePriority::Enhancement => println!("Enhancement!"),
        IssuePriority::Bug => println!("Bug!"),
        IssuePriority::Milestone(number: u8) => println!("Milstone #{}!", number),
        _ => println!("TODO!"),
    }
}
```

### Subslice patterns
```rust
let (start, end) = match attrs {
    [] => (0,0),
    [x0] => (x0, x0),
    [x0, .., xn] => (x0, xn),
};
```

### Or pattern
```rust
let Ok(x) | Err(x) = foo();
println!("Content of result {}", x);
```

And many more...

# Exercise
1. Model the `programmer`.
    1. Name
    2. Age
    3. Degree (Bachelor(Science, Arts, Business), Master, PHD)
        1. None
        2. Bachelor (Science, Arts, Business)
        3. Master (Science, Arts, Business)
        4. PHD (Type)
    4. State
        1. Eating
        2. Sleeping
        3. Programming
    5. Beverage
        1. Coffee (with coffeine)
        2. Beer (promille)
2. There are default parameters for
    1. Name: None
    2. Age: None
    3. Degree: None
    4. State: Programming
    5. Beverage: Coffee
3. Create two programmers and print them via `println!("{:?}, {:?}", programmer_1, programmer_2)`.
4. Write a function that will print the degree of a programmer without using `{:?}`.