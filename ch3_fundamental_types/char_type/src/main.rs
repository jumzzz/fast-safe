use std::mem;
use std::char;

// Why does it have many values
// Because Rust char is encoded as a Single Unicode Character
// 


#[test]
fn test_conversions() {
    assert_eq!(char::from_u32(78_u32), Some('N'));
    assert_eq!(char::from_u32(4_000_000), None);

}

#[test]
fn test_char_funcs() {
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

}



fn main() {
    let x = mem::size_of::<char>();
    println!("Size of a character = {:?}", x);

    let y : char = 'c';
    println!("Contents of y = {:?}", y);

    let z = '\u{CA0}';
    println!("Contents of z = {:?}", z);

    // This should be allowed
    let a0 = 'x' as u8;
    let a1 = 'x' as u16;
    let a2 = 'x' as u32;
    let a3 = 'x' as u64;
    let a4 = 'x' as i32;
    let a5 = 'x' as i64;

    // This is also allowed
    let a6 = 32_u8 as char;

    // This is not allowed
    // let b0 = 32_u16 as char;
    // char can be only casted from u8


    let char_symbol : u32 = 4_000_000;
    println!("Check option: {:?}", char::from_u32(78));
    println!("Check option: {:?}", char::from_u32(char_symbol));
    
    println!("Check option: {:?}", char::from_u32(0xD800 as u32));
    println!("Final Range of Unicode: {:?}", 0xD7FF as u32);


}
