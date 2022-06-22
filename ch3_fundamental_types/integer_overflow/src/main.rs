
#[test]
fn test_wrapping() {
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    assert_eq!(5_i16.wrapping_shl(17), 10);

}

#[test]
fn test_saturating() {
    assert_eq!(32760_i16.saturating_add(10), 32767);

}




fn main() {
    println!("result = {:?}", 10_u8.checked_add(20_u8));
    println!("result = {:?}", 100_u8.checked_add(200));
    println!("result = {:?}", 10_u8.checked_add(20_u8).unwrap());
    println!("result = {:?}", 100_u8.checked_add(200).unwrap());
}
