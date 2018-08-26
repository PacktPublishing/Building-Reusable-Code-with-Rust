pub trait Add<RHS = Self> { 
    //            ^----- Default generic
    type Output; // Associate type
    fn add(self, rhs: RHS) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,

}

impl Add for Point { // RHS is left as default
    type Output = Point; // Associate type

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
