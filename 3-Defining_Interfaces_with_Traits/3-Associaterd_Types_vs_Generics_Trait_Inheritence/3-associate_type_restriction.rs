use std::fmt;

trait Iterator {
    type Item: fmt::Display; // <= associate type

    fn next(&mut self) -> Option<Self::Item>; // <= used here
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Just for demo
        Some(42)
    }
}

/*
struct NoDisplayType; // Type without fmt::Display can't be used

impl Iterator for Counter {
    type Item = NoDisplayType; // Not OK here
    fn next(&mut self) -> Option<Self::Item> {
        // Just for demo
        Some("42".to_string())
    }
}
*/

fn main() {
    let mut counter = Counter {};

    let n = counter.next();
    println!("{}", n.unwrap());
    //        ^- using the default formatter requires fmt::Display
}
