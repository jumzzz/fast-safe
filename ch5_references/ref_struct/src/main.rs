struct S<'a> {
    r: &'a i32,
}

fn main() {
    let x: i32 = 32;
    let v = S {
        r: &x,
    };

    println!("S.r = {:?}", &v.r);
}