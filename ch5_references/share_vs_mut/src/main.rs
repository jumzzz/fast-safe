fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
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
    extend(&mut wave, &wave);
}
