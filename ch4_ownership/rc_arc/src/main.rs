use std::rc::Rc;

fn main() {
    // Also, Rc<T> assumes that T will be a shared-reference
    let s0: Rc<String> = Rc::new("corgi".to_string());
    let _s1: Rc<String> = s0.clone();    // This does not do copy. This adds to the reference counter of s0 
    let _s2: Rc<String> = s0.clone();     // This does not do copy. This adds to the reference counter of s0 
}
