#[derive(Debug, Copy, Clone)] // Automatically implmeneted by the compiler
struct MyStruct;

#[allow(unused_variables)]
pub fn main() {
    let x = MyStruct {};
    let y = x;
    // y is a copy of x, x is not moved
    println!("{:?}", x);
}
