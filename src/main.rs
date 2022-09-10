#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    Ok("some name".to_string())
    // Err(())
}

fn main() {
    get_user_name().expect_err("not failed to get user name");

    // println!("hello, {}", user_name)
}
