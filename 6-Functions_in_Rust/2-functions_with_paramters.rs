fn my_function_i32(msg: i32) {
    println!("Hello world {}", msg);
}

fn my_function_str(msg: &str) {
    println!("Hello world {}", msg);
}

fn main() {
    my_function_i32(42);
    my_function_str("Morty");
}
