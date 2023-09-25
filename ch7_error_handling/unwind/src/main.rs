fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}


fn main() {
    println!("pirate_share = {}", pirate_share(100, 20));
    println!("pirate_share = {}", pirate_share(100, 20));
    println!("pirate_share = {}", pirate_share(100, 20));
    println!("pirate_share = {}", pirate_share(100, 20));
    println!("pirate_share = {}", pirate_share(100, 0));

}
