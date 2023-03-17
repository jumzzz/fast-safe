# Breaking samples

## Breaking Samples v1
Compiling this piece of code will break because `x` doesn't live long enough
```rust
let r;
{
    let x = 1;
    r = &x;
}
assert_eq!(*r, 1);
```
Which yields to `E0597`

```bash
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     assert_eq!(*r, 1);
  |     ----------------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `breaking_samples` due to previous error
```

### How Rust Compiler Figure out that v1 is Broken?
- Rust compiler assign a reference type in your program called *lifetime* that meets the constraits on how its used.