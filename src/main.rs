#![deny(clippy::all)]

fn process_name(name: &str, callback: fn(&str) -> String) -> String {
    callback(name)
}

fn main() {
    let name_processed = process_name("john", |name: &str| name.to_uppercase());
    println!("{}", name_processed)
}
