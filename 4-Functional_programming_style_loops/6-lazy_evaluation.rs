fn main() {
    let numbers = vec![1, 2, 3, 4];

    numbers.iter().map(|x| x + 1);

    /*
    let plus_one_iter = numbers.iter().map(|x| x + 1);
    println!("{:?}", plus_one_iter);

    let plus_one:Vec<i32> = plus_one_iter.collect();
    println!("{:?}", plus_one);
    */
}
