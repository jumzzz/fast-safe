fn get_min<'a>(v: &Vec<&'a i32>) -> &'a i32 {
    v.iter().fold(&i32::MAX, |a,b| 
        std::cmp::min(a, *b)
     )
}

fn get_max(v: &Vec<&i32>) -> &i32 {
    v.iter().fold(&i32::MAX, |a,b| 
        std::cmp::max(a, *b)
     )
}


fn main() {
    let v0: Vec<&i32> = vec![&1, &2, &3, &4, &5];

    let min_val = get_min(&v0);
    println!("min of v = {}", &min_val);

    let v1: Vec<&i32> = vec![&1, &2, &3, &4, &5];

    let max_val = get_max(&v1);
    println!("min of v = {}", &max_val);

}
