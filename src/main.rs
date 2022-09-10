#![deny(clippy::all)]

fn get_first_name() -> Result<String, String> {
    // Ok("john".to_string())
    Err("there is no name".to_string())
}

fn get_last_name() -> Result<String, String> {
    Ok("lennon".to_string())
    // Err(())
}

fn get_full_name() -> Result<String, String> {
    let first_name: String = get_first_name()?;
    let last_name: String = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let full_name: Result<String, String> = get_full_name();
    let length = full_name.map_err(|err_msg| err_msg.len());

    println!("{:?}", length)
}
