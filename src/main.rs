#![deny(clippy::all)]

use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HassFullName {
    fn full_name(&self) -> String;
}

trait CanInitializeWithFullName {
    fn new(full_name: &str, age: u8) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str, age: u8) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age,
        }
    }
}

impl HassFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formater,
            "{} {}, {}",
            self.first_name, self.last_name, self.age
        )
    }
}

fn main() {
    let person: Person = Person::new("Leonardo Ferreira", 12);
    println!("{}", person)
}
