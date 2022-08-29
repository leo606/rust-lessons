#![deny(clippy::all)]

fn say_hello_world() -> String {
    // the last statement (without ';') will be automaticaly returned
    String::from("Hello world!")

    // 'return' keyword (with ';') can also be used, but is not a good practice
    // return String::from("Hello world!");
}

fn main() {
    let message: String = say_hello_world();
    println!("{}", message);
}
