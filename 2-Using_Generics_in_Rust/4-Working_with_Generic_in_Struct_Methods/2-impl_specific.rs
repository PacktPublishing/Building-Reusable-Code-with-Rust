struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
//  ^ notice that there is no <T> here
    fn get_x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.get_x()); // integer

    let p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.get_x()); // float
}
