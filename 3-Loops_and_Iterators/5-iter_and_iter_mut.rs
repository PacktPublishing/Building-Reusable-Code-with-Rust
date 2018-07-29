fn main() {
    let mut mylist = vec!(1, 2, 3, 4);
    
    for element in mylist.iter() {
        // &element is &i32
        println!("{}", *element);
    }

    println!("{}", mylist[0]);
    //             ^^^^^^ value used here after move

    for element in mylist.iter_mut() {
        // element is &mut i32
        *element = *element + 1;
        println!("{}", *element);
    }

    println!("{}", mylist[0]);
}
