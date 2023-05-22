### if let

#### Question
What makes `if let pattern` different with vanilla `if else` statements?

#### Dissecting the Question

Suppose we have

```rust
if let Some(cookie) = request.session_cookie {
    do_something(cookie)
}
```

Compared to this vanilla `if else` 

```rust
if request.session_cookie == Some(cookie) {
    do_something(request.session_cookie.unwrap())
}
```

Clearly, the former is more concise while the latter is more verbose. Not to mention this can mess up with borrowing rules. It's the Rust Dev's judgement call on when to use `if let` vs `if else`.