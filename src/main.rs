#![deny(clippy::all)]

fn main() {
    let mut values: Vec<i32> = vec![1, 2, 3];
    values.push(4);
    let four = values.pop();
}
