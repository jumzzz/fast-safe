#[allow(dead_code)]
fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

#[allow(dead_code)]
fn test_extend() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    println!("wave = {:?}", &wave);

    extend(&mut wave, &tail);
    println!("wave = {:?}", &wave);

    // This will not compile
    // Cannot borrow wave as immutable because it is also borrowed as mutable
    // rustc --explain E0502
    // extend(&mut wave, &wave);
}

#[allow(dead_code)]
fn shared_vs_mut() {
    let mut x = 10;
    let r1 = &x;                // shared-reference borrow here
    let r2 = &x;                // shared-reference borrow here
    // x += 10;                    // error: cannot borrow as mutable 
    // let m = &mut x;             // error: cannot borrow as mutable
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
}

#[allow(dead_code)]
fn multiple_mut() {
    let mut v = 100;
    let y0 = &mut v;
    // let y1 = &mut v;           // This will not compile

    *y0 += 1;
    *y0 += 1;                  // This will not work
    
    println!("v = {}", v);
}


fn main() {
    multiple_mut();
}
