# OOP in Rust
> Many competing definitions describe what OOP is; some definitions would classify Rust as object oriented, but other definitions would not.

## Objects hold data and define procedures that operate on that data and **encapsulation** that hides implementation details
```rust
pub struct SomeTuple {
    a: u32,
    b: u32
}

impl SomeTuple {
    pub fn sum(&self) -> u32 {
        self.get_a() + self.get_b()
    }

    fn get_a(&self) -> u32 {
        self.a
    }

    fn get_b(&self) -> u32 {
        self.b
    }
}
```

## Inheritance as a Type System and as Code Sharing
> Inheritance is a mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

> If a language must have inheritance to be an object-oriented language, then Rust is not one.

### Inheritance form of poor design choice
* Risk of sharing more code than necessary.
* Subclasses shouldn't always share all characteristics

### Polymorphism
> To many people, polymorphism is synonymous with inheritance. (...)

> Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called **bounded parametric polymorphism**.

## Trait objects
> Define a trait for **common** behavior.

> Inheritance for dynamic polymorphism is too instrusive, whereas trait objects are unobtrusive. ([source]((https://www.youtube.com/watch?v=VSlBhAOLtFA)))

### Example GUI components
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Implementing the trait for components
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

### Executing the code
```rust
use gui::Button;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```