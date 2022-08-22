#![deny(clippy::all)]

const NUMBER: i32 = 123; // constants declared upper case

fn main() {
    let name: &str = "leonardo";
    let age: i32 = 20;
    let age_complement: i32 = 4;
    println!("some constant number: {}", NUMBER);
    println!("age: {}", age + age_complement);
    println!("Hello, world!  {}.", name);
}
