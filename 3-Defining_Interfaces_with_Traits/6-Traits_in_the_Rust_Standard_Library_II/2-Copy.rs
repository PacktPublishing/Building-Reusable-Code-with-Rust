#[derive(Debug)] // ignore this for now
struct MyStruct;

// pub trait Copy: Clone {} // Clone is a supertrait of Copy

// Copy is trivial bitwise copy, can't be overloaded
// Notice it's empty, this is what we called marker trait
impl Copy for MyStruct {} 

// Clone is explicit memory copy, usually expensive
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}

#[allow(unused_variables)]
pub fn main() {
    let x = MyStruct {};
    let y = x;
    // y is a copy of x, x is not moved
    println!("{:?}", x);
}
