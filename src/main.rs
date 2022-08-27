#![deny(clippy::all)]

fn main() {
    let s1: String = String::from("john");
    let s2: &String = &s1;
    println!("hello, {}", s1);
    println!("hello, {}", s2)
}
