/*
enum Option<T> {
    Some<T>,
    None,
}
*/

fn first_element(list: Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        None
    } else {
        Some(list[0])
    }
}

fn main() {
    let list = vec!(1, 2, 3, 4);

    match first_element(list) {
        None => println!("Empty list!"),
        Some(x) => println!("The first element is {}", x),
    }

    match first_element(Vec::new()) {
        None => println!("Empty list!"),
        Some(x) => println!("The first element is {}", x),
    }
}
