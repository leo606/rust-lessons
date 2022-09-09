#![deny(clippy::all)]

fn main() {
    let age1: Option<i8> = Some(20);
    let age2: Option<i8> = Some(30);
    let age3: Option<i8> = Some(40);

    if let (Some(age_1), Some(age_2), Some(age_3)) = (age1, age2, age3) {
        println!("{} {} {}", age_1, age_2, age_3)
    }
}
