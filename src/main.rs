#![deny(clippy::all)]

use intutils::addition::add;

fn main() {
    let sum = add(2, 2);
    println!("{}", sum);
}
