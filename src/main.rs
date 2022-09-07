#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("foo", "bar");
    values.insert("name", "this is a name");

    if values.contains_key("name") {
        println!("contains name");
        println!("{}", values["name"])
    } else {
        println!("no name");
    }
}
