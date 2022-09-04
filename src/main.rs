#![deny(clippy::all)]

enum AnimalType { // enums always in pascal case
    Dog, // and the properties too
    Cat,
    Rabbit
}

fn main() {
    let my_dog: AnimalType = AnimalType::Dog;
}
