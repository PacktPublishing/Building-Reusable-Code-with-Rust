fn consuming_call<F>(func: F)
    where F: FnOnce() -> String // F is a FnOnce 
{
    println!("Consumed {}", func())
    //                      ^----- calling the F
}

fn main() {

    let x = String::from("hello");

    let return_x = move || x; // A closure that consumes x and returns it directly
    // FnOnce are automatically implemented for clouse that might consume captured variables
    consuming_call(return_x)
}

/*

pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    //                              ^--- may consume it, can only be called once
}

pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    //                             ^-------- may mutate it, 
    //                                       but safe to call multiple times
}

pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    //                         ^---- Safe to call multiple times 
    //                               without mutating state
}

*/
