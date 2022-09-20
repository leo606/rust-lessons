#![deny(clippy::all)]

use std::rc::Rc;

fn main() {
    let array = vec!["john".to_string(), "jane".to_string()];
    let rc = Rc::new(array);
    let rc_clone = rc.clone();
    let weak = Rc::downgrade(&rc);
    println!("{:?} {:?}", rc, weak.upgrade().unwrap());
    drop(rc);
    println!("{:?} {:?}", rc_clone, weak.upgrade().unwrap());
}
