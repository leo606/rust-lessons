#![deny(clippy::all)]

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut values: HashMap<&str, Person> = HashMap::new();

    let person: Person = Person {
        name: "name".to_string(),
        age: 1,
    };
    println!("{:?}", person);

    values.insert("personA", person);

    println!();
    println!();
    println!();
    println!("{:?}", values);
}
