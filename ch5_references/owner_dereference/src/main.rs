#[derive(Debug)]
struct Nice {
    x: u8,
    y: u8,
}

fn main() {
    let x_src = Nice {x: 5, y: 5}; 
    let x_ref = &x_src;

    println!("{:?}", *x_ref);
    println!("{:?}", *x_ref);

    // Meanwhile this transfer ownership
    let y_src = x_src;
    
    println!("{:?}", *x_ref);
    println!("{:?}", *x_ref);
}
