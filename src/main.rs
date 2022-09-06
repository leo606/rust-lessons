#![deny(clippy::all)]

fn main() {
    let values1: Vec<i32> = vec![1, 2, 3];
    if values1.contains(&9) {
        println!("true")
    } else {
        println!("false")
    }
}
