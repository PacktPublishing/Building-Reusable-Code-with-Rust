fn check_number(x: i32) {
    match x {
        n if n > 0  => println!("positive"),
        n if n == 0 => println!("zero"),
        n if n < 0  => println!("negative"),
        _           => unreachable!(),
        // thread 'main' panicked at 'internal error: entered unreachable code'
    }
}
fn main() {
    check_number(42);
}
