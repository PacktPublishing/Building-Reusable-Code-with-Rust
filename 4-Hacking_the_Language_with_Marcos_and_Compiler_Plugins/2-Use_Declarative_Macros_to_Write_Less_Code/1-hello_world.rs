macro_rules! hello {
    () => { // matches no argument
        // all call to "hello!()" will be expanded to this
        println!("hello");
    };
}

fn main() {
    hello!();
}
