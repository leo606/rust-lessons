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
        // matches patters has to cover all of enum type
        AnimalType::Dog => println!("is a dog"),
        _ => println!("some animal") // using _ (default), not covering all case
    }
}
