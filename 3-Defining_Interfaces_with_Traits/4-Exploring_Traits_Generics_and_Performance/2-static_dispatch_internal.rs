trait ShowMyself {
    fn show(&self) -> String;
}

impl ShowMyself for u32 {
    fn show(&self) -> String {
        format!("I'm u32 {}", *self)
    }
}

impl ShowMyself for String {
    fn show(&self) -> String {
        format!("I'm String {}", *self)
    }
}

/*
fn show_myself<T: ShowMyself>(x: T) {
    println!("{}", x.show());
}
*/

// Compiler "expands" the function into all implementations
fn show_myself_u32(x: u32) {
    println!("{}", x.show());
}

fn show_myself_string(x: String) {
    println!("{}", x.show());
}

fn main() {
    let x: u32 = 42;
    let y: String = "Hello World".to_string();

    // Compiler chooses the right implementation and inline it
    show_myself_u32(x);
    show_myself_string(y);
}
