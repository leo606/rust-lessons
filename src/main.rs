#![deny(clippy::all)]

fn main() {
    let values_array: [&str; 2] = ["foo", "bar"];
    println!("{}, {}", values_array[0], values_array[1])
}
