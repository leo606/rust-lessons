#![deny(clippy::all)]

fn main() {
    let mut values1: Vec<i32> = vec![1, 2, 3];
    let values2: Vec<i32> = vec![4, 5, 6];
    println!("values1 {:?}", values1);
    println!("values2 {:?}", values2);
    values1.extend_from_slice(&values2);
    println!("values1 {:?}", values1);
    println!("values2 {:?}", values2);
}
