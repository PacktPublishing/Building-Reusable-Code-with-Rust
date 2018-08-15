enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_integer = Option::Some(42);
    let some_float = Option::Some(3.14);
    let none: Option<i32> = Option::None;
}
