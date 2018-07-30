fn consuming_print(numbers: Vec<i32>) {
    //             ------- value moved into the function
    for number in numbers {
        println!("{}", number);
    }
}

fn main() {
    let numbers = vec!(1, 2, 3);
    consuming_print(numbers);
    //              ------- value moved here
    println!("{}", numbers[0]);
    //             ^^^^^^^ value used here after move

}
