struct Z<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let z = Z { x: &x, y: &y};
            r = z.x;
        }
    }
    println!("{}", r);
}
