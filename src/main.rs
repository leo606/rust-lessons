#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("foo", "bar");
    values.insert("name", "this is a name");

    for (&key, &value) in &values {
        println!("{}: {}", key, value);
    }
}
