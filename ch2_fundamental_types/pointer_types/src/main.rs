fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


fn check_references() {

    // References
    let x0: u8 = 20;
    let x1: u8 = 21;

    let y0 = &x0;
    let y1 = &x1;

    println!("value of x0 = {:p}", &x0);
    println!("value of x1 = {:p}", &x1);
    
    println!("value of y0 = {:p}", y0);
    println!("value of y1 = {:p}", y1);
    
    // Trying arithmetic

    let z0 = 10_u8 + x0;
    let z1 = 10_u8 + x1;

    println!("value of z0 = {}", z0);
    println!("value of z1 = {}", z1);
    
    print_type_of(&z0);
    print_type_of(&z1);


    /*
     * Output:
     * value of x0 = 0x7ffd284af486
     * value of x1 = 0x7ffd284af487
     * value of y0 = 0x7ffd284af486
     * value of y1 = 0x7ffd284af487
     
    Why Does the memory addresses seems adjacent?
    -   This is allocated in stack, that's why they're adjacent
    */


    // Function references
    // Not allowed
    //println!("function pointer = {:p}", print_type_of);
}

fn check_immutable_ref() {
    let x0: u8 = 20;

    println!("value = {}", x0);
}

fn check_shadowing() {
    let x = 5;
    println!("first value = {}", x);

    let x = x + 1;
    println!("second value (shadowing) = {}", x);

    {
        let x = 60;
        println!("third value (inner scope) = {}", x);
    }
    
    println!("fourth (shadowing) after scope = {}", x);

}

fn check_boxes_heap() {
    let t = (12, "eggs");
    let t_ref = &t;

    println!("t_ref = {:p}", t_ref);

    let b = Box::new(t);
    let b_ref = &b;

    println!("b_ref = {:p}", b_ref);
    println!("t_ref = {:p}", t_ref);

    // You need to manually move this

    let t = *b;
    println!("b_ref = {:p}", b_ref);
    println!("t_ref = {:p}", t_ref);
}


fn main() {
    check_references();
    check_immutable_ref();
    check_shadowing();
    
    println!("invoking check_boxes_heap()");
    check_boxes_heap();
}
