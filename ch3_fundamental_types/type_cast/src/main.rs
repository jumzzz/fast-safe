#[test]
fn test_type_casting() {
    assert_eq!( 10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!( -1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);
}

#[test]
fn test_type_casting_out_of_range() {
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

}

#[test]
fn test_integer_ops() {
    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

}


fn main() {
    println!("before casting to u8 = {} ", 1000_i16);
    println!("after casting to u8 = {}", 1000_i16 as u8);

    println!("modulo of 1000 % 256 = {}", 1000 % 256);

    //  When casting to integer types that are out-of-bounds,
    //  i.e. From larger integer type to smaller type, it results
    //  to a value of input % 2^N

    println!("{}", (-4 as i32).abs());
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

}
