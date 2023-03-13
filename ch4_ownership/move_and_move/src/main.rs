fn main() {
    let mut s0 = "Corgi";
    println!("heap_s0 = {:p}", s0);
    println!("stack_s0 = {:p}", &s0);
    
    let s1 = s0;
    println!("heap_s1 = {:p}", s1);
    println!("stack_s1 = {:p}", &s1);
    
    s0 = "Beagle";
    println!("heap_s0 = {:p}", s0);
    println!("stack_s0 = {:p}", &s0);
}
