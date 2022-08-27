#![deny(clippy::all)]

fn greet(name: &String) {
    println!("hello, {}!", name);
}

fn main() {
    let s1: String = String::from("john");
    let s2: &String = &s1;
    greet(&s1);
    greet(s2)
}
