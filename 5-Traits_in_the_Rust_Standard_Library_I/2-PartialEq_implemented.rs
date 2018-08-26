// '==' and '!='  operator: PartialEq
use std::cmp::PartialEq;

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

fn main() {
    let p1 = Point { x: 10, y: 10 };
    let p2 = Point { x: 10, y: 10 };

    println!("p1 == p2: {}", p1 == p2);

    let p3 = Point { x: 20, y: 30 };

    println!("p1 != p3: {}", p1 != p3);
}
