#![deny(clippy::all)]

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
}

fn main() {
    let point: Point = Point(6.2, 3.2, 3.2);
    let point_two_times = point.twice();
    println!("{}, {}, {}", point_two_times.0, point_two_times.1, point_two_times.2);
}
