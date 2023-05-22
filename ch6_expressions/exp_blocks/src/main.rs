fn main() {
    let v1 = {
        let x = 10;
        let y = 20;
        x + y
    };

    let v2 = {
        let x = 30;
        let y = 50;
        y - x
    };

    let v3 = {
        if v1 > v2 {
            v1 * v1
        }
        else {
            v2 * v2
        }
    };

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v3 = {} {}", v3, v1 * v1 == v3);
}
