struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
//  ^^^      ^^^
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.get_x());
}
