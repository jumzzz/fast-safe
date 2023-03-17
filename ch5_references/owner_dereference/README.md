# Ownership Transfer and Derefencing

### Question
- Does Dereferencing Transfer Ownership?

### Answer
- Yes, but you have to be very explicit if you want to.
- In case of passing by value (with dereferencing), let say in `print!` it doesn't do implicit moves if you perform dereferencing.
- In case of the explicit assignments, like `let y_src = *x_ref`, moves happens.

In this code, **No Transfer of Ownership Happens on Line 11 and Line 12**
```rust
#[derive(Debug)]
struct Nice {
    x: u8,
    y: u8,
}

fn main() {
    let x_src = Nice {x: 5, y: 5}; 
    let x_ref = &x_src;

    println!("{:?}", *x_ref);
    println!("{:?}", *x_ref);
}
```

Meanwhile, assigning `x_src` to `y_src` will violate Ownership rules because move happens at that instance. Also, you cannot move a variable that outlives a shared reference.

```rust
#[derive(Debug)]
struct Nice {
    x: u8,
    y: u8,
}

fn main() {
    let x_src = Nice {x: 5, y: 5}; 
    let x_ref = &x_src;

    println!("{:?}", *x_ref);      // For some reason this does not implicitly perform move
    println!("{:?}", *x_ref);

    // Move happens here but this won't compile since moving x_src cannot outlive x_ref
    // If x_ref can outlive x_src, x_ref will become a dangling pointer.
    let y_src = *x_ref;      // rustc --explain E0505
}