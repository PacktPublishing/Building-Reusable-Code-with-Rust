struct PointInteger {
    x: i32,
    y: i32,
}

struct PointFloat {
    x: f32,
    y: f32,
}

fn main() {
    let integer = PointInteger { x: 5, y: 10 };
    let float = PointFloat { x: 1.0, y: 4.0 };
}
