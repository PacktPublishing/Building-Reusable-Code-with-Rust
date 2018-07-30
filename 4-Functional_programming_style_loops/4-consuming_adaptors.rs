fn main() {
    let numbers = vec![1, 2, 3, 4];

    let sum:i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let max:&i32 = numbers.iter().max().unwrap();
    println!("Max: {}", max);

}
