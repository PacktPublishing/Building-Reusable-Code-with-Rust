fn main() {
    let mylist = vec!(1, 2, 3, 4);
    
    for element in mylist.into_iter() {
        println!("{}", element);
    }

    println!("{}", mylist[0]);
    //             ^^^^^^ value used here after move

}
