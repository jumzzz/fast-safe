# Reference of Reference (Ad-Infinitum)
- You can also chain references by `&&..&T`
- Uncovering their values leads directly to the main referant, but you can also view their chain of addresses

## Chaining References Code
```rust
fn main() {
    let v: u8 = 8;

    let r: &u8 = &v;
    let rr: &&u8 = &r;
    let rrr: &&&u8 = &rr;
    let rrrr: &&&&u8 = &rrr;
    let rrrrr: &&&&&u8 = &rrrr;

    let d = *rrrrr;
    let dd = *d;
    let ddd = *dd;
    let dddd = *ddd;
    let ddddd = *dddd;

    println!("Printing Just Values...");
    println!("value of r = {}", r);
    println!("value of rr = {}", rr);
    println!("value of rrr = {}", rrr);
    println!("value of rrrr = {}", rrrr);
    println!("value of rrrrr = {}", rrrrr);
    
    println!("{}", d);
    println!("{}", dd);
    println!("{}", ddd);
    println!("{}", dddd);
    println!("{}", ddddd);
    println!("");

    println!("Printing Just Addresses...");
    println!("addr of v = {:p}", &v);
    println!("addr of r = {:p}", r);
    println!("addr of rr = {:p}", rr);
    println!("addr of rrr = {:p}", rrr);
    println!("addr of rrrr = {:p}", rrrr);
    println!("addr of rrrrr = {:p}", rrrrr);

}
```

## Output of the Code
- The value of each `r{n}` unpacks to the main referant directly
- The addresses of each `r{n}` are stored in the stack. That's why it was reflected when you try to print each addresses.
```bash
Printing Just Values...
value of r = 8
value of rr = 8
value of rrr = 8
value of rrrr = 8
value of rrrrr = 8
8
8
8
8
8

Printing Just Addresses...
addr of v = 0x7ffcc77962af
addr of r = 0x7ffcc77962af
addr of rr = 0x7ffcc77962b0
addr of rrr = 0x7ffcc77962b8
addr of rrrr = 0x7ffcc77962c0
addr of rrrrr = 0x7ffcc77962c8
```
