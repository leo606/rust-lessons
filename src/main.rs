#![deny(clippy::all)]

fn main() {
    let name: Option<&str> = None;

    match name {
        Some(name) => println!("{}", name),
        None => println!("a horse with no name")
    }
}
