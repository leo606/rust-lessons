#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person: Person = Person {
        name: "layne".to_string(),
        age: 30,
    };
    println!("the person age {}, and name is {}", person.age, person.name)
}
