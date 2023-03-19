# Shared VS Mutable

Supposed that we have a piece of code
```rust
fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
```

This works fine on the following
```
let mut wave = Vec::new();
let head = vec![0.0, 1.0];
let tail = [0.0, -1.0];

extend(&mut wave, &head);
println!("wave = {:?}", &wave);

extend(&mut wave, &tail);
println!("wave = {:?}", &wave);
```

But when it comes to self-referencing -- This one breaks
```rust
// This will not compile
// Cannot borrow wave as immutable because it is also borrowed as mutable
// rustc --explain E0502
extend(&mut wave, &wave);
```

Though Rust prevents you from doing this. Since it explicitly tells you that you cannot borrow references that's both mutable and immutable.

```bash
error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable
  --> src/main.rs:21:23
   |
21 |     extend(&mut wave, &wave);
   |     ------ ---------  ^^^^^ immutable borrow occurs here
   |     |      |
   |     |      mutable borrow occurs here
   |     mutable borrow later used by call

For more information about this error, try `rustc --explain E0502`.
error: could not compile `share_vs_mut` due to previous error
```
In other words, **those two reference lifetimes must not overlap**.