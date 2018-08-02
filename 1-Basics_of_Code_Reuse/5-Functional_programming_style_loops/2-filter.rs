fn main() {
    let numbers = vec![1, 2, 3, 4];

    let larger_then_two = numbers.into_iter().filter(|&x| x > 2);
    // [    1,     2,    3,    4]
    // [false, false, true, true]
    // => [3, 4]

    larger_then_two.for_each(|x| println!("{}", x));
}
