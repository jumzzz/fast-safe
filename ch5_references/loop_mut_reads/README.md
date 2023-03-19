# Looping on both Shared Reference VS Mutable Reference

### Question
- If you're doing a loop that mutates a particular variable and you want to read how that variable mutates -- How are you going to place the reads?

### Answer
- Always after the line where mutation occurs.

### Code Sample

```rust
let vals = Vec::from_iter(1..=10);
let mut total_sum = 0;

for v in &vals {
    let mut_ref = &mut total_sum;

    // Wrong placement of shared reference
    // You need to place this after the mutation ended
    // let shared_ref = &total_sum;             // Won't compile
    // println!("total_sum = {}", shared_ref);  // Won't compile

    *mut_ref += v;

    // Right placement of shared reference
    // Reading values should be placed after mutation        
    let shared_ref = &total_sum;
    println!("total_sum = {}", shared_ref);
} 
```