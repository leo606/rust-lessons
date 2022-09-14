#![deny(clippy::all)]

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HassFullName {
    fn full_name(&self) -> String;
}

impl HassFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {}
