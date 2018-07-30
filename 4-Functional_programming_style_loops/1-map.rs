fn main() {
    let numbers = vec![1,2,3,4];

    let plus_one = numbers.iter().map(|x| x + 1);

    plus_one.for_each(|x| println!("{}", x));
}
