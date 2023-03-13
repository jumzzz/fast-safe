fn main() {
    let x0 = vec![1,2,3,4,5,6];

    let v0 = &x0[0];
    let v1 = &x0[1];

    println!("v0 = {}", v0);
    println!("v1 = {}", v1);

    // This won't compile
    println!("x0[0] = {}", x0[0]);
    println!("x0[1] = {}", x0[1]);

}
