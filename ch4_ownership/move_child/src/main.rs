// This finally compiles
struct Dog<'a> {
    name: &'a str,                      // ensures the name is always owned by Dog struct
    physique: &'a str,                  // ensures the physique is always owned by Dog struct
}

fn main() {
    let dog = Dog {
        name: "Corgi",
        physique: "Short",
    };

    let _name = dog.name;               // Moves from dog.name -> _name
    let _physique = dog.physique;       // Moves from dog.physique -> _physique

    println!("dog.name = {:p}", dog.name);
    println!("_name = {:p}", dog.name);
}
