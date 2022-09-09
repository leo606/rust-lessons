#![deny(clippy::all)]

fn main() {
    let name: Option<&str> = None;

    let name_wrapped = name.unwrap_or_else(|| {
        "default name"
    });

    println!("{}", name_wrapped)
}
