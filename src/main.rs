#![deny(clippy::all)]

// lifetime rules
// #1 - if you have parameters to your function that is references, the compiler automaticaly assings a lifetime operator to each one of them
// #2 - single input lifetime is assigned to all outputs
// #3 - if &self or &mut self in parameters, lifetime of self is assigned to output

fn main() {

}
