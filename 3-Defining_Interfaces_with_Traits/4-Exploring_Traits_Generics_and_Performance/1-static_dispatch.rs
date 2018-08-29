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

fn show_myself<T: ShowMyself>(x: T) {
    println!("{}", x.show());
}

fn main() {
    let x: u32 = 42;
    let y: String = "Hello World".to_string();

    show_myself(x);
    show_myself(y);
}
