# Reference of Closures/Functions
You can also take references of Functions
```rust
fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

fn main() {
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}
```