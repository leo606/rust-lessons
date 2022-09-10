#![deny(clippy::all)]

fn get_first_name() -> Result<String, ()> {
    // Ok("john".to_string())
    Err(())
}

fn get_last_name() -> Result<String, ()> {
    Ok("lennon".to_string())
    // Err(())
}

fn get_full_name() -> Result<String, ()> {
    let first_name: String = get_first_name()?;
    let last_name: String = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let full_name: Result<String, ()> = get_full_name();
    let length = full_name.map(|name| name.len()).unwrap_or_default();

    println!("{}", length)
}
