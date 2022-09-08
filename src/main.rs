#![deny(clippy::all)]

fn main() {
    let values: Vec<&str> = vec!["john", "jane", "mary", "bob", "tom"];

    for name in values.into_iter().filter(|n: &&str| n.len() == 3) {
        println!("{}", name)
    }
}
