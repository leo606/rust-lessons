#![deny(clippy::all)]

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

fn main() {
    let age: BoxedValue<u8> = BoxedValue::new(2);
}