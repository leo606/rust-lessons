#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("foo", "bar");
    values.insert("name", "this is a name");

    println!("{:?}", values);

    if values.contains_key("foo") {
        println!("contains 'foo'")
    } else {
        println!("does not contains 'foo'")
    }

    values.remove("foo");

    if values.contains_key("foo") {
        println!("contains 'foo'")
    } else {
        println!("does not contains 'foo'")
    }
}
