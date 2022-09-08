#![deny(clippy::all)]

use std::slice::Iter;

fn main() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];

    let values_iter: Iter<i32> = values.iter();

    let values_sum: i32 = values_iter.sum();
    let values_sum_two: i32 = values_iter.sum(); // iterators cannot be double consumed

    println!("{}", values_sum);
}
