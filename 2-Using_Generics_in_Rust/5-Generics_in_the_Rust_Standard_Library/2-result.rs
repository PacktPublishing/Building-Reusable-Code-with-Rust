/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn get_middle(list: Vec<i32>) -> Result<i32, &'static str> {
    match list.len() {
        0 => Err("empty list"),
        x if x % 2 == 0 => Err("list has even number of elements"),
        _ => Ok(list[list.len() / 2]),
    }
}

fn main() {
    println!("{:?}", get_middle(vec![1, 2, 3]));
    println!("{:?}", get_middle(vec![1, 2, 3, 4]));
    println!("{:?}", get_middle(Vec::new()));
}
