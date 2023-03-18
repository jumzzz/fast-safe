# Returning Reference

## Another Lifetime Example
We have a weird syntax here
```rust
fn get_min<'a>(v: &Vec<&'a i32>) -> &'a i32 {
    v.iter().fold(&i32::MAX, |a,b| 
        std::cmp::min(a, *b)
     )
}

fn main() {
    let v: Vec<&i32> = vec![&1, &2, &3, &4, &5];

    let min_val = get_min(&v);
    println!("min of v = {}", &min_val);
}

```
- The lifetime parameter `'a` ensures that the lifetime within `v[i]` will retain its lifetime even until you unpacked the minimum value. 

## Sure. What will happen if we remove the lifetime annotation.
This sample code will not compile
```rust
fn get_max(v: &Vec<&i32>) -> &i32 {
    v.iter().fold(&i32::MAX, |a,b| 
        std::cmp::max(a, *b)
     )
}

fn main() {
    let v1: Vec<&i32> = vec![&1, &2, &3, &4, &5];

    let max_val = get_max(&v1);
    println!("min of v = {}", &max_val);
}

```
```bash
error[E0106]: missing lifetime specifier
 --> src/main.rs:7:30
  |
7 | fn get_max(v: &Vec<&i32>) -> &i32 {
  |               ----------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `v`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
  |
7 | fn get_max<'a>(v: &'a Vec<&'a i32>) -> &'a i32 {
  |           ++++     ++      ++           ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `return_ref` due to previous error
```

## Further Reitaration
- The rule of thumb is that reference must not outlive the referant. If you're returning a reference from within a certain scope like vector elements or a member of the struct and you want to ensure that the lifetime within that scope was preserved -- you can explicitly annotate that particular lifetime with `'a`.