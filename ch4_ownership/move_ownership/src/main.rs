fn main() {
    println!("Moving Heap values by assignment to heap_v0, heap_v1, heap_v2, and heap_v3");
    let heap_v0 = Box::new(5);
    println!("heap_addr_v0 = {:p}", heap_v0);
    let heap_v1 = heap_v0;
    println!("heap_addr_v1 = {:p}", heap_v1);
    let heap_v2 = heap_v1;
    println!("heap_addr_v2 = {:p}", heap_v2);
    let heap_v3 = heap_v2;
    println!("heap_addr_v3 = {:p}", heap_v3);

    println!("...");

    let x0: u64 = 5;
    let x1: u64 = 10;
    let x2: u64 = 3;

    println!("Moving Heap values by assignment to heap_y0, heap_y1, heap_y2, and heap_y3");
    println!("stack_addr_x0 = {:p}", &x0);
    println!("stack_addr_x1 = {:p}", &x1);
    println!("stack_addr_x2 = {:p}", &x2);

    let y0: u64 = 5;
    let y1: u64 = 10;
    let y2: u64 = 3;

    println!("stack_addr_y0 = {:p}", &y0);
    println!("stack_addr_y1 = {:p}", &y1);
    println!("stack_addr_y2 = {:p}", &y2);
}
