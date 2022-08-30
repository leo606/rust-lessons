#![deny(clippy::all)]

fn main() {
    let multiply_by_two = |x: i32| x * 2;
    let ptr = multiply_by_two;
    let result = ptr(12);
    print!("{}", result)
}
