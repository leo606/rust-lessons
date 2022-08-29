#![deny(clippy::all)]

fn say_hello_world(to_person: String) -> String {
    format!("hello, {}!", to_person)
}

fn main() {
    let hello: String = say_hello_world(String::from("leo"));
    println!("{}", hello)
}
