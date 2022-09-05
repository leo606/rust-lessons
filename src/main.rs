#![deny(clippy::all)]

#[derive(PartialEq)]

struct Point(f64, f64);

enum Shapes {
    Circle { radius: f64, center: Point },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let rectagle: Shapes = Shapes::Rectangle {
        width: 3.0,
        height: 4.0,
    };

    if let Shapes::Rectangle {
        width: 3.4,
        height: 3.4,
    } = rectagle
    {
        println!("is equal")
    } else {
        println!("is not equal")
    }
}
