#![deny(clippy::all)]

fn main() {
    let values: [&str; 3] = ["value0", "value1", "value2"];
    println!("length: {}", values.len())
}
