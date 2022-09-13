#![deny(clippy::all)]

// lifetime rules
// #1 - if you have parameters to your function that is references, the compiler automaticaly assings a lifetime operator to each one of them
// #2 - single input lifetime is assigned to all outputs
// #3 - if &self or &mut self in parameters, lifetime of self is assigned to output

struct Person<'a> {
    first_name: &'a str, // these two strings will live on for as long as that instance of person lives
    last_name: &'a str,
}

impl<'a> Person<'a> {
    // implementations of structs that have lifetime operators, also needs to have lifetimes operators
    fn first_char_of_first_name(&self) -> &str {
        // output will live as long self lives
        &self.first_name[0..1]
    }

    fn get_full_name(&self) -> String { // return value is not a referense, so no need for lifetime operator, return value will live as long the func caller needs it to live
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {}
