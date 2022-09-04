#![deny(clippy::all)]

#[derive(PartialEq)]

enum AnimalType { // enums always in pascal case
    Dog, // and the properties too
    Cat,
    Rabbit
}

fn main() {
    let my_dog: AnimalType = AnimalType::Dog;
    if my_dog == AnimalType::Dog {
        println!("is a dog")
    }
}
