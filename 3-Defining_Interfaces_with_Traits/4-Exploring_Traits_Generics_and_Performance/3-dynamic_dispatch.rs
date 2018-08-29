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

fn show_myself(x: &ShowMyself) {
    println!("{}", x.show());
}

fn main() {
    let x: &ShowMyself = &42;
    let y = &"Hello World".to_string() as &ShowMyself;

    show_myself(x);
    show_myself(y);
}
