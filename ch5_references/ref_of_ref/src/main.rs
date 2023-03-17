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
