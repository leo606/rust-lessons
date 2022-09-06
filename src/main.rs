#![deny(clippy::all)]

fn main() {
    let values_array: [&str; 2] = ["foo", "bar"];
    for value in values_array.iter() {
        println!("{}", value)
    }
}
