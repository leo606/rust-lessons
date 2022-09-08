#![deny(clippy::all)]

fn main() {
    let values: Vec<&str> = vec!["john", "jane", "mary", "bob", "tom"];

    for name in values.iter() {
        if name.len() != 3 {
            continue;
        }
        println!("{}", name)
    }
}
