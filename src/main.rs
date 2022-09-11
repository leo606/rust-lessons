#![deny(clippy::all)]

fn get_random_name<'lt1, 'lt2>(a: &'lt1 str, b: &'lt2 str) -> &'lt1 str { // adding "&'static" the string will live for the entirety of the application
    a
}

fn main() {
    let random_name: &str = get_random_name("john", "doe");
    println!("hello, {}", random_name)
}
