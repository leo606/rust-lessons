#![deny(clippy::all)]

// fn greet(name: &String) {
//     println!("hello, {}!", name);
// }

fn empty_string(value: &mut String) {
    // clear the content of the input string
    value.clear()
}

fn main() {
    let mut name: String = String::from("john");
    println!("{}", name);
    empty_string(&mut name);
    println!("{}", name)
}
