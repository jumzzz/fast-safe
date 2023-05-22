use rand::prelude::*;

#[allow(dead_code)]
fn break_return() {

    let mut v: Vec<i32> = (1..=100).collect();
    let mut rng = thread_rng();

    v.shuffle(&mut rng);

    let answer = loop {
        if *v.iter().next().unwrap() > 50 {
            let val = v.iter().next().unwrap();
            break *val;
        }
        else {
            break 0;
        }
    };

    println!("answer = {}", answer);
}

#[allow(dead_code)]
fn break_outside() {

    let mut v: Vec<Vec<i32>> = vec![
        (1..=100).collect(),
        (1000..=1100).collect(),
        (1..=100).collect(),
        (1..=100).collect(),
        (1..=100).collect(),
    ];
    
    let mut rng = thread_rng();
    v.shuffle(&mut rng);

    'outer:
    for vv in &v {
        println!("{:?}", &vv);
        for vvv in vv {
            if *vvv >= 1000 {
                println!("\nValues >=1000 detected. Breaking...");
                break 'outer;
            }
        }
        println!("");
    }
}


fn main() {
    break_outside();
}
