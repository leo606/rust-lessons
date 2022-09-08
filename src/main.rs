#![deny(clippy::all)]

fn main() {
    let value: Option<i32> = Some(10);
    let name = Option::<&str>::None;
}
