fn main() {
    println!("Hello, world!");
    let user1: User = User {
        name: String::from("Yan"),
        age: 32,
        race: Race::Dark,
    };
    println!("{:#?}", user1);
    dbg!(user1);
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    race: Race,
}

#[derive(Debug)]
enum Race {
    Dark,
    White,
}
