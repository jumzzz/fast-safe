#[allow(dead_code)]
fn sample_v1() {

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    println!("v0 = {:?}\n", &v);
    
    let fifth = v.pop().unwrap();
    println!("after v.pop().unwrap()");
    println!("v1 = {:?}", &v);
    println!("fifth = {:?}\n", fifth);

    let second = v.swap_remove(1);
    println!("after v.swap_remove(1)");
    println!("v2 = {:?}", &v);
    println!("second = {:?}\n", second);
    
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    println!("after std::mem::replace(&mut v[2], \"substitute\".to_string())");
    println!("v3 = {:?}", &v);
    println!("third = {:?}", third);
}

#[allow(dead_code)]
fn sample_v2() {

    let x = vec!["1", "2", "3", "4"];
    // This needs &x
    for xx in &x {
        println!("{}", xx);
    }
    println!("{:?}", &x);
}

#[allow(dead_code)]
fn sample_v3() {
    let mut v = vec![
        Some("x1".to_string()),
        Some("x2".to_string()),
        Some("x3".to_string()),
    ];

    let v0 = std::mem::replace(&mut v[0], None);
    let v1 = std::mem::replace(&mut v[1], None);
    let v2 = std::mem::replace(&mut v[2], None);

    println!("v = {:?}", &v);
    println!("v0 = {:?}", &v0);
    println!("v1 = {:?}", &v1);
    println!("v2 = {:?}", &v2);
}


fn main() {
    sample_v3();

}
