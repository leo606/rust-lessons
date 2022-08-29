#![deny(clippy::all)]

fn main() {
    let say_hello_to = |name: &str| format!("Hello, {} ", name);
    println!("{}", say_hello_to("fulano"))
}
