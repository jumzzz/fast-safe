fn main() {
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
