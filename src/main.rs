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
    let name_ref_immutable: &String = &name;
    let mut name_ref_muttable: &mut String = &mut name;
    println!("{}", name);
    println!("{}", name_ref_immutable);
    println!("{}", name_ref_muttable);
    // you cannot reference a variable as mutable while there are imotable references
}
