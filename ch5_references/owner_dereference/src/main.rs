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

    let xx = x_ref.x;
    let yy = x_ref.y;

    // Meanwhile this transfer ownership
    // let y_src = *x_ref;
    
    // println!("{:?}", *x_ref);
    // println!("{:?}", *x_ref);
}
