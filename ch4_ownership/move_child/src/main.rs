struct Dog {
    name: String,
    physique: String,
}

fn main() {
    let dog = Dog {
        name: String::from("Corgi"),
        physique: String::from("Short"),
    };

    let _name = dog.name;               // Moves from dog.name -> _name
    let _physique = dog.physique;       // Moves from dog.physique -> _physique
}
