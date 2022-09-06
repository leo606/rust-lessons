#![deny(clippy::all)]

fn main() {
    let mut values1: Vec<i32> = vec![1, 2, 3];
    if !values1.is_empty() {
        println!("empty")
    } else {
        println!("not empty")
    }
    values1.clear();
    if values1.is_empty() {
        println!("empty")
    } else {
        println!("not empty")
    }
}
