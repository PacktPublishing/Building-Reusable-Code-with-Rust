#[derive(Debug)] // ignore this for now
struct MyStruct;

#[allow(unused_variables)]
pub fn main() {
    let x = MyStruct {};
    let y = x;
    // x has moved into y, x is not usable anymore
    println!("{:?}", x);
}
