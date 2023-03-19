# References in Struct
You cannot simply assign an arbitrary reference inside a struct. This is obvious for following reason -- Assigning Multiple References inside structs exposes your code with ownership ambiguities. That's why the compiler complains.

Hence, we have to be explicit on who will own the lifetime of the reference when assigned inside the struct. We can do this by lifetime annotations.


```rust
struct S<'a> {
    r: &'a i32,
}

fn main() {
    let x: i32 = 32;
    let v = S {
        r: &x,
    };

    println!("S.r = {:?}", &v.r);
}
```

## From the book Programming Rust: Fast, Safe Systems Development Chapter 5
> Turning back to the preceding code, the expression `S { r: &x }` creates a fresh `S` value with some lifetime `'a`. When you store `&x` in the `r` field, you constrain `'a` to lie entirely within `x`'s lifetime.

In other words, in our code sample, the lifetime of the reference that you assigned in the field of struct will remain within the original referant.


## Struct within a Struct
If you have a struct within a struct and you want to apply lifetime annotations, simply do the following:

```
struct S<'a> {
    r: &'a i32,
}

struct D<'a> {
    s: S<'a>,
}

fn main() {
    let x: i32 = 32;

    let d = D {
        s: S {
            r: &x,
        },
    };

    println!("S.r = {:?}", &d.s.r);
}
```