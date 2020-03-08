# Traits
> Traits are similar to a feature often called interfaces in other languages, although with some differences.

## Example: Issue
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for issue {
    fn summarize(&self) -> String {
        format!("{}: {}", self.id, self.title)
    }
}
```

## Default implementations
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## Traits as parameters
```rust
pub fn notify(item: impl Summary + Clone) {
    println!("Breaking news! {}", item.clone().summarize());
}
```

## Returning Types that implement traits
```rust
fn returns_summarizable() -> impl Summary {
    Issue::default()
}
```