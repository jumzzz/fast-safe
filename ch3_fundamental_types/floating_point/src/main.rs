#[test]
fn test_float_inf() {
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);
    assert_eq!(f32::MIN, -f32::MAX);

}

#[test]
fn test_ops() {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
}

fn main() {
    println!("Hello, world!");

    println!("PI Value = {:?}", std::f32::consts::PI);
    println!("PI Value = {:?}", std::f64::consts::PI);

    println!("sqrt of 2 = {}", f64::sqrt(2.0));
}
