#![deny(clippy::all)]

fn say_hello_world() { // the function does not have any return value (could be typed with '-> ()')
    let message: String = String::from("hello world");
    println!("{}", message);
}

fn main() {
    say_hello_world();
}
