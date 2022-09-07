#![deny(clippy::all)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("foo", "bar");
    values.insert("name", "this is a name");

    let entry: Entry<&str, &str> = values.entry("foo");

    match entry {
        std::collections::hash_map::Entry::Occupied(value) => println!("{:?}", value.get()),
        _ => println!("not found")
    }
}
