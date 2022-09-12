#![deny(clippy::all)]

struct Person<'a> { // must define how long references will live for
    name: &'a str,
}

fn main() {

}
