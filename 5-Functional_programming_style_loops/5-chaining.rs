fn main() {
    let numbers = vec![1, 2, 3, 4];

    let squared_sum:i32 = numbers.iter()
                                 .map(|x| x * x)  // squared
                                 .sum();

    println!("Squared sum: {}", squared_sum);
}
