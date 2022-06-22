#[test]
fn test_arrays_v1() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

}

#[test]
fn test_arrays_v2() {
    let mut sieve = [true; 10_000];

    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;

            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);
}

#[test]
fn test_arrays_sort() {
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1,2,3,4,5]);
}

#[test]
fn test_vector_v1() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
        
}

#[test]
fn test_vector_v2() {
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);
}

#[test]
fn test_vector_v3() {
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
}

#[test]
fn test_vector_v4() {
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();

    assert_eq!(palindrome, vec!["panama", "a canal", "a plan","a man"]);
}

#[test]
fn test_vector_v5() {
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);

    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
}

#[test]
fn test_vector_v6() {
    let mut v = vec![10, 20, 30, 40, 50];

    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}

#[test]
fn test_vector_v7() {
    let mut v = vec!["Snow Puff", "Glass Gem"];

    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);
}


/*
    A Vec<T> consists of three values:

    1. A pointer to heap-allocated buffer for elements.
    2. The number of elements that buffer has to "capacity" to store.
    3. The number that it actually contains now

*/
fn vector_three_values() {

    println!("Invoking vector_three_values()");

    let mut v: Vec<i32> = (0..5).collect();
    println!("Vector pointer = {:p}", v.as_ptr());
    println!("Vector capacity = {:?}", v.capacity());
    println!("Vector length = {:?}", v.len());

    v.push(5);
    v.push(5);
    v.push(5);
    
    println!("Vector capacity = {:?}", v.capacity());
    println!("Vector length = {:?}", v.len());

}



fn slices_demo() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    println!("Slice value sv = {:?}", &sv);
    println!("Slice value sa = {:?}", &sa);

    println!("Slice value sa [0..2] = {:?}", &sa[0..2]);
    println!("Slice value sv [1..3]= {:?}", &sv[1..3]);
    
}


fn main() {
    vector_three_values();
    slices_demo();
}
