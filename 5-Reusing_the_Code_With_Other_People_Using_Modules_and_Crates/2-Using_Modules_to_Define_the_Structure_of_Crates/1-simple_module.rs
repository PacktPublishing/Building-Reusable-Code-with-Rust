mod cat {
    fn meow() { // Should be "pub fn", we'll talk about this later
    }
}

fn main() {
    cat::meow();
}
