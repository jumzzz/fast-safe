fn main() {
    let x0 = vec![
        String::from("Corgi"), 
        String::from("Beagle"), 
        String::from("Daschund")
    ];

    let v0 = &x0[0];
    let v1 = &x0[1];

    println!("v0 = {}", v0);
    println!("v1 = {}", v1);

    // This won't compile
    println!("x0[0] = {}", x0[0]);
    println!("x0[1] = {}", x0[1]);

    let y0 = Box::new(5);
    let y1 = &y0;
    let _y2 = *y1;      // You cannot move out of shared reference

}
