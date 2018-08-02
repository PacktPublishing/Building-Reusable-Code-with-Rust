fn main() {

    for element in 1..10 {
        println!("{}", element);
    }

    let mylist = [1, 2, 3, 4];

    for element in mylist.into_iter() {
        println!("{}", element);
    }

    /*
    for element in mylist {
    //             ^^^ Not an iterator
        println!("{}", element);
    }
    */
}
