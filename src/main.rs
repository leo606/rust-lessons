#![deny(clippy::all)]

fn main() {
    let mut values: Vec<i32> = vec![1, 2, 3];
    values.push(4);
    println!("values are {:?}", values);
    values.extend_from_slice(&[5, 5, 5]);
    println!("values are {:?}", values);
}
