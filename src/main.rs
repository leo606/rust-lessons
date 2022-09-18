#![deny(clippy::all)]

use std::ops::Deref;

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let age: BoxedValue<u8> = BoxedValue::new(2);
    let age_derefered = age.deref(); // deref method returns a reference to the value
    let actual_age = *age; // * gives the actual u8 value, not a reference
    let age_pointer = *(age.deref());
    println!("{}", age_pointer)
}