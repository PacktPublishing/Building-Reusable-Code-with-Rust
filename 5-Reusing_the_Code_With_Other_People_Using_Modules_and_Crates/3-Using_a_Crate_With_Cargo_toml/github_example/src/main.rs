extern crate rand; // compile and link the rand crate

fn main() {
    let x: u8 = rand::random();
    println!("{}", x);
}
