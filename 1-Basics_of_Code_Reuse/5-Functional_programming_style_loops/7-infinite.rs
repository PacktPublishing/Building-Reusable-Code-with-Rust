fn main() {

    let numbers:Vec<i32> = (1..)  // 1 to inifintiy
        .map(|x| x + 1)  // [2, 3,  4,  5,  6, ...]
        .map(|x| x * x)  // [4, 9, 16, 25, 36, ...]
        .take(5)  // "take" the first five
        .collect();  // collect them into a vec

    println!("{:?}", numbers);
}
