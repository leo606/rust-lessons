#![deny(clippy::all)]

fn main() {
    let name: Option<&str> = Some("john");

    let name_unsafely_unrapped = name.expect("a horse with no name");

    println!("{}", name_unsafely_unrapped);
}
