#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    // Ok("some name".to_string())
    Err(())
}

fn main() {
    let is_ok: bool = get_user_name().is_ok();
    let is_err: bool = get_user_name().is_err();

    println!("{} {}", is_ok, is_err)
}
