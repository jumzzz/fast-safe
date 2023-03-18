# Reference As Parameters

### Violating Exclusive Reference Rules
This code won't compile

**Sample Code**
```rust
static mut STASH: &i32 = &5;
fn f(p: &i32) { STASH = p; }

fn main() {
    f(&3);
}
```
**Error Message**
```bash
error: lifetime may not live long enough
 --> src/main.rs:2:17
  |
2 | fn f(p: &i32) { STASH = p; }
  |         -       ^^^^^^^^^ assignment requires that `'1` must outlive `'static`
  |         |
  |         let's call the lifetime of this reference `'1`

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
 --> src/main.rs:2:17
  |
2 | fn f(p: &i32) { STASH = p; }
  |                 ^^^^^^^^^ use of mutable static
  |
  = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `ref_as_parameters` due to 2 previous errors
```

- This violates the rule that there should be only single exclusive reference that should be allowed.
- Also, this is not thread safe since multiple concurrent values can mutate `STASH`. And Rust Compiler is pretty defensive against this.

Even if you add the unsafe block.

```rust
static mut STASH: &i32 = &5;
fn f(p: &i32) { 
    unsafe {
        STASH = p;
    } 
```
The compiler still catches that assignment of stash doesn't outlive static.

```bash
2 | fn f(p: &i32) { 
  |         - let's call the lifetime of this reference `'1`
3 |     unsafe {
4 |         STASH = p;
  |         ^^^^^^^^^ assignment requires that `'1` must outlive `'static`
```

### The Fix
```rust
fn f(p: &'static i32) {...}
```
- This explicitly tells the compiler that the reference has a lifetime of a `static` variable.
- Recall that static variables default lifetime lives throughout the lifetime of the whole program itself.