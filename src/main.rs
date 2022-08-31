#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}

fn get_person(name: String, age: u8) -> Person {
    Person { name, age }
}

fn main() {
    let person = get_person("layne".to_string(), 90);
    println!("the person age {}, and name is {}", person.age, person.name)
}
