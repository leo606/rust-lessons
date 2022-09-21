#![deny(clippy::all)]

use std::cell::Cell;

struct Person {
    name: String,
    age: Cell<u8>,
}

impl Person {
    fn increment_age(&self) {
        self.age.set(self.age.get() + 1);
    }
}

fn main() {
    let person = Person {
        name: "john".to_string(),
        age: Cell::new(20),
    };

    let age = person.age.get();

    println!("{}", age);

    person.increment_age();

    let new_age = person.age.get();

    println!("{}", new_age);
}
