#![deny(clippy::all)]

#[derive(Debug)]

struct Point(f64, f64, f64);

impl Point {
    fn describe(&self) {
        println!("point is at ({}, {}, {})", self.0, self.1, self.2)
    }

    fn x_value(&self) -> f64 {
        self.0
    }

    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn twice_mut(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
}

fn main() {
    let point: Point = Point(6.2, 3.2, 3.2);
    println!("{:?}", point)
}
