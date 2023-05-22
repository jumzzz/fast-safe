### Break in Loops

### Breaking the Outside Loop

One interesting use of lifetimes is it allows you to outer loops inside of the loop itself.

```rust
#[allow(dead_code)]
fn break_outside() {

    let mut v: Vec<Vec<i32>> = vec![
        (1..=100).collect(),
        (1000..=1100).collect(),
        (1..=100).collect(),
        (1..=100).collect(),
        (1..=100).collect(),
    ];
    
    let mut rng = thread_rng();
    v.shuffle(&mut rng);

    'outer:
    for vv in &v {
        println!("{:?}", &vv);
        for vvv in vv {
            if *vvv >= 1000 {
                println!("\nValues >=1000 detected. Breaking...");
                break 'outer;
            }
        }
        println!("");
    }
}
```