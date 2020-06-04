#[test]
fn test_add_with_zero()
{ // Cargo fmt fix
    // Arrange
    let a = 42;
    let b = 0;

    // Act
    let result = add(a, b);

    // Asster
    assert_eq!(result, a);
}

#[test]
fn test_add_commutativity() {
    // Arrange
    let a = 35;
    let b = 7;

    // Act + Assert
    assert_eq!(add(a, b), add(b, a));
}

/// # Addition
/// Adds two unsigned integers a and b
/// ## Example
/// ```
/// let a = 7;
/// let b = 35;
/// let result = add(a,b);
/// ```
pub fn add(a: u32, b: u32) -> u32 {
    // return a + b; // Clippy warning
    a + b
}

fn main() {
    let a = 7;
    let b = 35;

    println!("Sum of {} + {} = {}", a, b, add(a, b));
}
