mod animal {
    pub fn move_around() {}

    pub mod cat {
        pub fn meow() {}
    }

    mod dog {
        fn bark() {}
    }
}

fn main() {
    animal::move_around();
    animal::cat::meow();
    animal::dog::bark(); // error[E0603]: module `dog` is private
}
