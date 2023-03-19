struct S<'a> {
    r: &'a i32,
}

struct D<'a> {
    s: S<'a>,
}

fn main() {
    let x: i32 = 32;

    let d = D {
        s: S {
            r: &x,
        },
    };

    println!("S.r = {:?}", &d.s.r);
}