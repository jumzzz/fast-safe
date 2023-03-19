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

## Fundamental Rules Between Shared Reference VS Exclusive Reference
**Shared access is read-only access**
- There can be multiple Shared Reference but there can be no existing mutable access at the same time.

**Mutable access is exclusive access**
- There could be only one Mutable References. The only way that you can have multiple reference is to let others borrow that mutable access.

## Another example

Here's another example that demonstrates that shared-reference VS exclusive-reference cannot exist simultaneously.

```rust
let mut x = 10;
let r1 = &x;                // shared-reference borrow here
let r2 = &x;                // shared-reference borrow here
x += 10;                    // error: cannot borrow as mutable 
let m = &mut x;             // error: cannot borrow as mutable
println!("r1 = {}", r1);
println!("r2 = {}", r2);
```
Which throws a following error.

```bash
error[E0506]: cannot assign to `x` because it is borrowed
  --> src/main.rs:30:5
   |
28 |     let r1 = &x;                // shared-reference borrow here
   |              -- borrow of `x` occurs here
29 |     let r2 = &x;                // shared-reference borrow here
30 |     x += 10;                    // error: cannot borrow as mutable 
   |     ^^^^^^^ assignment to borrowed `x` occurs here
31 |     let m = &mut x;             // error: cannot borrow as mutable
32 |     println!("r1 = {}", r1);
   |                         -- borrow later used here

For more information about this error, try `rustc --explain E0506`.
```