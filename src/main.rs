#![deny(clippy::all)]

fn main() {
    let mut counter: i16 = 0;
    loop {
        println!("{}", counter);
        counter += 1;
    }
}
