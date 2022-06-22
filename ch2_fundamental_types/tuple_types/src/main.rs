// Test for no return value
fn no_return_value() {
    // Intended to be empty
}


#[test]
fn test_tuple_split_v1() {
    let text = "I see the eigenvalue in thine eye";

    let (head, tail) = text.split_at(21);

    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}


#[test]
fn test_tuple_split_v2() {
    
    let text = "I see the eigenvalue in thine eye";

    let temp = text.split_at(21);

    let head = temp.0;
    let tail = temp.1;

    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_no_return_value() {
    // () is called unit type sometimes
    assert_eq!(no_return_value(), ());

    // Testing the std::mem::swap function
    let mut u0: u8 = 23;
    let mut u1: u8 = 21;

    assert_eq!(std::mem::swap(&mut u0, &mut u1), ());
}





fn main() {
    println!("Hello, world!");
}
