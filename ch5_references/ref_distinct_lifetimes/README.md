# Distinct Lifetimes

Supposed that we have a code like this

```rust
struct Z<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let z = Z { x: &x, y: &y};
            r = z.x;
        }
    }
    println!("{}", r);
}

```
If you try to compile this, `rustc` complains with the following error.

```bash
error[E0597]: `y` does not live long enough
  --> src/main.rs:12:35
   |
12 |             let z = Z { x: &x, y: &y};
   |                                   ^^ borrowed value does not live long enough
...
15 |     }
   |     - `y` dropped here while still borrowed
16 |     println!("{}", r);
   |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
```

Remember that annotating `'a` contraints itself by the lifetime that you passed by the struct. Which implies that it assumes that both `x` and `y` won't outlive each other.

To make the lifetimes of both `x` and `y` distinct, we can do the following.

```rust
struct Z<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let z = Z { x: &x, y: &y};
            r = z.x;
        }
    }
    println!("{}", r);
}
```

Making lifetime annotations for both `x` and `y` separate will let the compiler stop complaining. Hence, it can safely drop the ownership of y when it goes out of scope.

With this definition, `z.x` and `z.y` have **independent lifetimes.**