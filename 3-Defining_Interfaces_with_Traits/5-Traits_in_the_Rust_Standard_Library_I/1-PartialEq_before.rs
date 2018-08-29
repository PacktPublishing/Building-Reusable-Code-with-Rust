// '==' and '!='  operator: PartialEq

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 10 };
    let p2 = Point { x: 10, y: 10 };

    println!("p1 == p2: {}", p1 == p2);
    //                          ^- How does the equal operator work?
}
