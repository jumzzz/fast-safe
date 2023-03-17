fn breaking_v1() {
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }
    // Adding this will make this break
    // assert_eq!(*r, 1);
}


fn main() {
    breaking_v1();
}
