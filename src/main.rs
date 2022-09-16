#![deny(clippy::all)]

fn main() {
    let age: Box<i32> = Box::new(30);
    let twice: i32 = *age * 2;
    println!("{}", twice)
}