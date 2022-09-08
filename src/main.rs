#![deny(clippy::all)]

use std::slice::Iter;

fn main() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];

    let values_iter: Iter<i32> = values.iter();

    let values_double: Vec<i32> = values_iter.map(|value: &i32| value * 2).collect();

    println!("{:?}", values_double);
}
