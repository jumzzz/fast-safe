use std::cmp::min;

fn gcd(x: i32, y: i32) -> i32 {
    let max_den = min(x, y);
    for i in (1..=max_den).rev() {
        if x % i == 0 && y % i == 0 {
            return i
        } 
    }
    1
}


fn main() {
    let v1 = gcd(36, 9);
    let v2 = gcd(45, 18);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
}
