#![deny(clippy::all)]

fn main() {
    // Box is a reference to a value in the heap
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("great stuff");

    match value {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Error")
    }
}
