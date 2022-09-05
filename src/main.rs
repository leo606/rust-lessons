#![deny(clippy::all)]

#[derive(PartialEq)]

enum AnimalType {
    // enums always in pascal case
    Dog, // and the properties too
    Cat,
    Rabbit,
}

fn main() {
    let my_dog: AnimalType = AnimalType::Cat;
    match my_dog {
        AnimalType::Dog => println!("is a dog"),
        AnimalType::Cat => println!("is a cat"),
        AnimalType::Rabbit => println!("is a rabbit"),
    }
}
