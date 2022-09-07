#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();

    values.insert("dog", "lyra");
    
    values.entry("cat").or_insert("noo");

    println!("{:?}", values)
}
