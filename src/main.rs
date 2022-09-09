#![deny(clippy::all)]

fn main() {
    let age: Option<i32> = Some(2);
    let age_multiplied_by_two = age.map(|age| age * 2);
    println!("{}", age_multiplied_by_two.unwrap_or_default())

}
