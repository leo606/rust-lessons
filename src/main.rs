#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("foo", "bar");
    values.insert("name", "this is a name");

    let bar = match values.get("foo") {
        Some(value) => value,
        None => "not found"
    };

    println!("{}", bar)
}
