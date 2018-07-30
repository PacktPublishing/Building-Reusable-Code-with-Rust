fn consuming_print(numbers: &Vec<i32>) {
    //                      ^ use reference instead
    for number in numbers {
        println!("{}", number);
    }
}

fn main() {
    let numbers = vec!(1, 2, 3);
    consuming_print(&numbers);
    //              ^ pass a reference to the Vec<i32>
    println!("{}", numbers[0]);
    //             ^^^^^^^ not moved, we can use it again

}
