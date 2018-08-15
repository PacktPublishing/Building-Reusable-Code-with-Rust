struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
//  ^^^      ^^^
    fn x(&self) -> &T {
        &self.x
    }
}


impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
