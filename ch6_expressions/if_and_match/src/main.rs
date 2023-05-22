use rand::seq::SliceRandom;

#[allow(dead_code)]
fn match_v1() {
    let code = 1;

    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code)
    };
}

#[allow(dead_code)]
enum Card {
    Jack,
    Queen,
    King,
    Ace,
}

#[allow(dead_code)]
fn match_v2() {
    let card = vec![
        Card::Jack,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];


    let score = match card
                     .choose(&mut rand::thread_rng())
                     .unwrap() 
    {
        Card::Jack => 10,
        Card::Queen => 11,
        Card::King => 12,
        Card::Ace => 13,
    };

    println!("score = {}", score);
}

#[allow(dead_code)]
fn match_v3() {

    let v = [true, false]; 

    let best_sports_team = if *v.choose(&mut rand::thread_rng())
                              .unwrap() { "Predators" } else { "Roar" };
    println!("{}", best_sports_team );
}

fn main() {
    // match_v2();
    match_v3();
}
