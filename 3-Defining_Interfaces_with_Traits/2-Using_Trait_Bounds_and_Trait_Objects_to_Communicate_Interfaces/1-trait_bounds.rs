fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    //                ^------ Copy
    for &item in list.iter() {
    //  ^---- Copy
        if item > largest {
        //      ^ PartialOrd::gt()
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T 
//     where T: PartialOrd + Copy {
//     // function body
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // T is explicitly specified
    let result = largest::<i32>(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // T can be inferred by the compiler
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
