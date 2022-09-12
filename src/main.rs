#![deny(clippy::all)]

// fn get_fist_name<'a>(full_name: &'a str) -> &'a str {
//     full_name
// }

fn get_fist_name(full_name: &str) -> &str { // rust compiler automaticaly assign lifetime operators to parameter and return value
    full_name
}

fn main() {

}
