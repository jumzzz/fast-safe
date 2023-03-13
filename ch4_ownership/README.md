# Chapter 4: Ownership

## Moves

We have a Rust code here that wants to understand what exactly Rust means by "Moving Ownership"

```rust
let heap_v0 = Box::new(5);
let heap_v1 = heap_v0;
let heap_v2 = heap_v1;
let heap_v3 = heap_v2;


let x0: u64 = 5;
let x1: u64 = 10;
let x2: u64 = 3;

let y0: u64 = 5;
let y1: u64 = 10;
let y2: u64 = 3;
```

Which produces an output

```bash
Moving Heap values by assignment to heap_v0, heap_v1, heap_v2, and heap_v3
heap_addr_v0 = 0x5603f6a4ead0
heap_addr_v1 = 0x5603f6a4ead0
heap_addr_v2 = 0x5603f6a4ead0
heap_addr_v3 = 0x5603f6a4ead0
...
Moving Heap values by assignment to heap_y0, heap_y1, heap_y2, and heap_y3
stack_addr_x0 = 0x7ffcd6205868
stack_addr_x1 = 0x7ffcd6205870
stack_addr_x2 = 0x7ffcd6205878
stack_addr_y0 = 0x7ffcd6205988
stack_addr_y1 = 0x7ffcd6205990
stack_addr_y2 = 0x7ffcd6205998
```
Which clears certain ambiguities
- The data that's contained from a particular memory block aren't literally moved. It's the ownership that was being moved. It's like an individual land owner owning a piece of land and then he sells it to another individual. What's being "moved" is the ownership, and not the actual piece of data.
- Assignment at stack-based values are copied and not moved. This is evident on how stack addresses are incremented.
