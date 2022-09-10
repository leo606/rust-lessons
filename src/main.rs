#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    // Ok("some name".to_string())
    Err(())
}

fn main() {
    let user_name = get_user_name().expect("failed to get user name");

    println!("hello, {}", user_name)
}
