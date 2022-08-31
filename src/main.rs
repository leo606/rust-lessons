#![deny(clippy::all)]

struct Point(f64, f64, f64);

fn main() {
    let point: Point = Point(3.2, 3.2, 3.2);
    println!("x = {} y = {} z = {}", point.0, point.1, point.2);
}
