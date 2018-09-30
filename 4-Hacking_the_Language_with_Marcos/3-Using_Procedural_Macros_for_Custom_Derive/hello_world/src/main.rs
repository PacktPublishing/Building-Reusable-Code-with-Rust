extern crate hello_macro; // define the HelloMacro trait
#[macro_use]
extern crate hello_macro_derive; // define the derive

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Cat;

/* The #[derive(HelloMacro)] will implement this:
impl HelloMacro for Cat {
    fn hello_macro() {
        println!("Hello, Macro! I'm a Cat!");
    }
}
*/

fn main() {
    Cat::hello_macro();
}
