// Note: Fold is an accumulator that lets you perform accumulation process for a,b
// Example: (1..0).fold(0, |a,b| a + b) sums all the value with initial value of zero

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

fn main() {
    let v = factorial(5);
    println!("v = {}", &v + &5);

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}
