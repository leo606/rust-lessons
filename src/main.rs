#![deny(clippy::all)]

fn main() {
    let name: Option<&str> = Some("sdf");

    let b: bool = name.is_some();

    println!("{}", b)
}
