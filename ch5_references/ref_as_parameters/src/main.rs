static mut STASH: &i32 = &5;
fn f(p: &'static i32) { 
    unsafe {
        STASH = p;
    } 
}

fn main() {
    f(&3);
}
