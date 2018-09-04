trait Iterator {
    type Item; // <= associated type

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

impl Iterator for Counter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Just for demo
        Some("42".to_string())
    }
}

fn main() {
    let mut counter = Counter {};

    let n = counter.next();
    println!("{:?}", n);
}
