#![deny(clippy::all)]

fn main() {
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    println!("{}", full_name("john", "lennon"))
}
