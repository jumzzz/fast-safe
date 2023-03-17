# Chapter 5 - References

## Two Kinds of Reference
1. **Shared Reference**
- Lets you read but not modify its referent.
- It allows you to have many shared references to a particular value at a time as you like.

2. **Mutable Reference/Exclusive Reference**
- Lets you to both read/write its referent.
- You're only allowed to have one Exclusive Reference at a time.

Its **Multiple Reader/Single Writer** rule at a compile time. Also, **References are nonowning pointers**.


### Personal Comments
- Rust is pretty explicit on how you indicate passing by references and passing by value. This forces the programmer to pay more attention on how data was being moved. This is not so clear in higher languages like Python.
- Also, indicating a difference between shared references VS mutable references makes it more natural to guard against unintended side-effects like the following code snippet

**Without Side Effects**
```rust
fn sort_vec(target_vec: &Vec<u8>) {
    ...
}
```

**With Side Effects**
```rust
fn sort_vec(target_vec: &mut Vec<u8>){
    ...
}
```