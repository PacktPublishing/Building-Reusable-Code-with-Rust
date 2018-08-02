fn main() {
    let numbers = vec![1, 2, 3, 4];

    let y = 1;

    let plus_one = numbers.iter().map(|x| x + y);

    plus_one.for_each(|x| println!("{}", x));
}
