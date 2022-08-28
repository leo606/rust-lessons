#![deny(clippy::all)]

// fn greet(name: &String) {
//     println!("hello, {}!", name);
// }

// fn empty_string(value: &mut String) {
//     // clear the content of the input string
//     value.clear()
// }

fn get_name() -> &String {
    &"John".to_string()
}

fn main() {
    let name = get_name();
}
