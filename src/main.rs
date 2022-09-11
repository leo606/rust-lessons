#![deny(clippy::all)]

fn get_full_name() -> &'static str { // adding "&'static" the string will live for the entirety of the application
    "john lenno"
}

fn main() {
    let full_name: &str = get_full_name();
    println!("hello, {}", full_name)
}
